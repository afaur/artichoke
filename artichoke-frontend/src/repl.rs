//! A REPL (read–eval–print–loop) for an artichoke interpreter exposed by
//! the [`artichoke-backend`](artichoke_backend) crate.
//!
//! The REPL is readline enabled, but does not save history. The REPL supports
//! multi-line Ruby expressions, CTRL-C to break out of an expression, and can
//! inspect return values and exception backtraces.

use ansi_term::Style;
use artichoke_backend::eval::Context;
use artichoke_backend::exception::{Exception, RubyException};
use artichoke_backend::gc::MrbGarbageCollection;
use artichoke_backend::{Artichoke, BootError};
use artichoke_core::eval::Eval;
use artichoke_core::value::Value;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::io::{self, Write};

use crate::parser::{self, Parser, State};

const REPL_FILENAME: &[u8] = b"(airb)";

#[cfg(test)]
mod filename_test {
    #[test]
    fn repl_filename_has_no_nul_bytes() {
        assert_eq!(
            None,
            super::REPL_FILENAME
                .iter()
                .copied()
                .position(|b| b == b'\0')
        );
    }
}

/// REPL errors.
#[derive(Debug)]
pub enum Error {
    /// Fatal error.
    Fatal,
    /// Could not initialize REPL.
    ReplInit,
    /// Unrecoverable [`Parser`] error.
    ReplParse(parser::Error),
    /// Error during Artichoke interpreter initialization.
    Artichoke(BootError),
    /// Exception thrown by eval.
    Ruby(Exception),
    /// IO error when writing to output or error streams.
    Io(io::Error),
}

/// Configuration for the REPL readline prompt.
pub struct PromptConfig {
    /// Basic prompt for start of a new expression.
    pub simple: String,
    /// Altered prompt when an expression is not terminated.
    pub continued: String,
    /// Prefix for the result of `$expression.inspect`. A newline is printed
    /// after the Ruby result.
    pub result_prefix: String,
}

impl Default for PromptConfig {
    #[must_use]
    fn default() -> Self {
        Self {
            simple: ">>> ".to_owned(),
            continued: "... ".to_owned(),
            result_prefix: "=> ".to_owned(),
        }
    }
}

fn preamble(interp: &mut Artichoke) -> Result<String, Error> {
    let description = interp
        .eval(b"RUBY_DESCRIPTION")
        .map_err(Error::Ruby)?
        .try_into::<&str>(interp)
        .map_err(BootError::from)
        .map_err(Error::Artichoke)?;
    let compiler = interp
        .eval(b"ARTICHOKE_COMPILER_VERSION")
        .map_err(Error::Ruby)?
        .try_into::<&str>(interp)
        .map_err(BootError::from)
        .map_err(Error::Artichoke)?;
    let mut buf = String::new();
    buf.push_str(description);
    buf.push('\n');
    buf.push('[');
    buf.push_str(compiler);
    buf.push(']');
    Ok(buf)
}

/// Run a REPL for the mruby interpreter exposed by the `mruby` crate.
pub fn run(
    mut output: impl Write,
    mut error: impl Write,
    config: Option<PromptConfig>,
) -> Result<(), Error> {
    let config = config.unwrap_or_else(Default::default);
    let mut interp = artichoke_backend::interpreter().map_err(Error::Artichoke)?;
    writeln!(output, "{}", preamble(&mut interp)?).map_err(Error::Io)?;

    let parser = Parser::new(&mut interp).ok_or(Error::ReplInit)?;
    // safety:
    // Context::new_unchecked requires that REPL_FILENAME have no NUL bytes.
    // REPL_FILENAME is controlled by this crate and asserts this invariant
    // with a test.
    interp.push_context(unsafe { Context::new_unchecked(REPL_FILENAME.to_vec()) });
    // TODO expose a lineno API
    // unsafe {
    //     let api = interp.0.borrow();
    //     (*api.ctx).lineno = 1;
    // }

    let mut rl = Editor::<()>::new();
    // If a code block is open, accumulate code from multiple readlines in this
    // mutable `String` buffer.
    let mut buf = String::new();
    let mut parser_state = State::default();
    loop {
        // Allow shell users to identify that they have an open code block.
        let prompt = if parser_state.is_code_block_open() {
            config.continued.as_str()
        } else {
            config.simple.as_str()
        };

        let readline = rl.readline(prompt);
        match readline {
            Ok(line) => {
                buf.push_str(line.as_str());
                parser_state = parser.parse(buf.as_str()).map_err(Error::ReplParse)?;
                if parser_state.is_code_block_open() {
                    buf.push('\n');
                    continue;
                }
                match interp.eval(buf.as_bytes()) {
                    Ok(value) => {
                        let result = value.inspect(&mut interp);
                        output
                            .write_all(config.result_prefix.as_bytes())
                            .map_err(Error::Io)?;
                        output.write_all(result.as_slice()).map_err(Error::Io)?;
                    }
                    Err(exc) => {
                        if let Some(backtrace) = exc.backtrace(&mut interp) {
                            writeln!(
                                error,
                                "{} (most recent call last)",
                                Style::new().bold().paint("Traceback")
                            )
                            .map_err(Error::Io)?;
                            for (num, frame) in backtrace.into_iter().enumerate().rev() {
                                write!(error, "\t{}: from ", num + 1).map_err(Error::Io)?;
                                error.write_all(frame.as_slice()).map_err(Error::Io)?;
                                writeln!(error).map_err(Error::Io)?;
                            }
                        }
                        write!(
                            error,
                            "{} {}",
                            Style::new().bold().paint(exc.name()),
                            Style::new().bold().paint("(")
                        )
                        .map_err(Error::Io)?;
                        Style::new()
                            .bold()
                            .underline()
                            .paint(exc.message())
                            .write_to(&mut error)
                            .map_err(Error::Io)?;
                        writeln!(error, "{}", Style::new().bold().paint(")")).map_err(Error::Io)?;
                    }
                }
                for line in buf.lines() {
                    rl.add_history_entry(line);
                    // TODO: expose lineno API
                    // unsafe {
                    //     let api = interp.0.borrow();
                    //     (*api.ctx).lineno += 1;
                    // }
                }
                // mruby eval successful, so reset the REPL state for the
                // next expression.
                interp.incremental_gc();
                buf.clear();
            }
            // Reset the buf and present the user with a fresh prompt
            Err(ReadlineError::Interrupted) => {
                // Reset buffered code
                buf.clear();
                // clear parser state
                parser_state = State::default();
                writeln!(output, "^C").map_err(Error::Io)?;
                continue;
            }
            // Gracefully exit on CTRL-D EOF
            Err(ReadlineError::Eof) => break,
            Err(_) => return Err(Error::Fatal),
        };
    }
    Ok(())
}
