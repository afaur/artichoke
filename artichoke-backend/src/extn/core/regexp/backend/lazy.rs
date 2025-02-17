use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::fmt;

use crate::extn::core::regexp::{Config, Encoding, Regexp, RegexpType};
use crate::extn::prelude::*;

use super::{NameToCaptureLocations, NilableString};

#[derive(Debug)]
pub struct Lazy {
    literal: Config,
    encoding: Encoding,
    regexp: OnceCell<Regexp>,
}

impl Lazy {
    #[must_use]
    pub fn new(literal: Config) -> Self {
        Self {
            literal,
            encoding: Encoding::default(),
            regexp: OnceCell::new(),
        }
    }

    pub fn regexp(&self, interp: &Artichoke) -> Result<&Regexp, Exception> {
        self.regexp.get_or_try_init(|| {
            Regexp::new(
                interp,
                self.literal.clone(),
                self.literal.clone(),
                self.encoding,
            )
        })
    }
}

impl fmt::Display for Lazy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        string::format_unicode_debug_into(f, self.literal.pattern.as_slice())
            .map_err(string::WriteError::into_inner)
    }
}

impl Clone for Lazy {
    fn clone(&self) -> Self {
        Self::new(self.literal.clone())
    }
}

impl RegexpType for Lazy {
    fn box_clone(&self) -> Box<dyn RegexpType> {
        Box::new(self.clone())
    }

    fn captures(
        &self,
        interp: &Artichoke,
        haystack: &[u8],
    ) -> Result<Option<Vec<NilableString>>, Exception> {
        self.regexp(interp)?.inner().captures(interp, haystack)
    }

    fn capture_indexes_for_name(
        &self,
        interp: &Artichoke,
        name: &[u8],
    ) -> Result<Option<Vec<usize>>, Exception> {
        self.regexp(interp)?
            .inner()
            .capture_indexes_for_name(interp, name)
    }

    fn captures_len(
        &self,
        interp: &Artichoke,
        haystack: Option<&[u8]>,
    ) -> Result<usize, Exception> {
        self.regexp(interp)?.inner().captures_len(interp, haystack)
    }

    fn capture0<'a>(
        &self,
        interp: &Artichoke,
        haystack: &'a [u8],
    ) -> Result<Option<&'a [u8]>, Exception> {
        self.regexp(interp)?.inner().capture0(interp, haystack)
    }

    fn debug(&self) -> String {
        let mut debug = String::from("/");
        let mut pattern = String::new();
        // Explicitly supress this error because `debug` is infallible and
        // cannot panic.
        //
        // In practice this error will never be triggered since the only
        // fallible call in `string::format_unicode_debug_into` is to `write!` which never
        // `panic!`s for a `String` formatter, which we are using here.
        let _ = string::format_unicode_debug_into(&mut pattern, self.literal.pattern.as_slice());
        debug.push_str(pattern.replace("/", r"\/").as_str());
        debug.push('/');
        debug.push_str(self.literal.options.modifier_string().as_str());
        debug.push_str(self.encoding.string());
        debug
    }

    fn literal_config(&self) -> &Config {
        &self.literal
    }

    fn derived_config(&self) -> &Config {
        &self.literal
    }

    fn encoding(&self) -> &Encoding {
        &self.encoding
    }

    fn inspect(&self, interp: &Artichoke) -> Vec<u8> {
        self.regexp(interp)
            .map(|regexp| regexp.inner().inspect(interp))
            .unwrap_or_default()
    }

    fn string(&self, interp: &Artichoke) -> &[u8] {
        self.regexp(interp)
            .map(|regexp| regexp.inner().string(interp))
            .unwrap_or_default()
    }

    fn case_match(&self, interp: &mut Artichoke, haystack: &[u8]) -> Result<bool, Exception> {
        self.regexp(interp)?.inner().case_match(interp, haystack)
    }

    fn is_match(
        &self,
        interp: &Artichoke,
        haystack: &[u8],
        pos: Option<Int>,
    ) -> Result<bool, Exception> {
        self.regexp(interp)?.inner().is_match(interp, haystack, pos)
    }

    fn match_(
        &self,
        interp: &mut Artichoke,
        haystack: &[u8],
        pos: Option<Int>,
        block: Option<Block>,
    ) -> Result<Value, Exception> {
        self.regexp(interp)?
            .inner()
            .match_(interp, haystack, pos, block)
    }

    fn match_operator(
        &self,
        interp: &mut Artichoke,
        haystack: &[u8],
    ) -> Result<Option<Int>, Exception> {
        self.regexp(interp)?
            .inner()
            .match_operator(interp, haystack)
    }

    fn named_captures(&self, interp: &Artichoke) -> Result<NameToCaptureLocations, Exception> {
        self.regexp(interp)?.inner().named_captures(interp)
    }

    fn named_captures_for_haystack(
        &self,
        interp: &Artichoke,
        haystack: &[u8],
    ) -> Result<Option<HashMap<Vec<u8>, NilableString>>, Exception> {
        self.regexp(interp)?
            .inner()
            .named_captures_for_haystack(interp, haystack)
    }

    fn names(&self, interp: &Artichoke) -> Vec<Vec<u8>> {
        self.regexp(interp)
            .map(|regexp| regexp.inner().names(interp))
            .unwrap_or_default()
    }

    fn pos(
        &self,
        interp: &Artichoke,
        haystack: &[u8],
        at: usize,
    ) -> Result<Option<(usize, usize)>, Exception> {
        self.regexp(interp)?.inner().pos(interp, haystack, at)
    }

    fn scan(
        &self,
        interp: &mut Artichoke,
        haystack: Value,
        block: Option<Block>,
    ) -> Result<Value, Exception> {
        self.regexp(interp)?.inner().scan(interp, haystack, block)
    }
}
