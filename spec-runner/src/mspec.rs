//! Embedded `MSpec` framework.

use artichoke_backend::exception::Exception;
use artichoke_backend::{Artichoke, ConvertMut, Eval, LoadSources, TopSelf, ValueLike};

/// Load `MSpec` sources into the Artichoke virtual filesystem.
///
/// # Errors
///
/// If an exception is raised on the Artichoke interpreter, it is returned.
pub fn init(interp: &mut Artichoke) -> Result<(), Exception> {
    for source in Sources::iter() {
        if let Some(content) = Sources::get(&source) {
            interp.def_rb_source_file(source.as_bytes(), content)?;
        }
    }
    Ok(())
}

/// `MSpec` source code.
#[derive(RustEmbed)]
#[folder = "vendor/mspec/lib"]
pub struct Sources;

/// Load the Artichoke `MSpec` entry point end execute the specs.
///
/// # Errors
///
/// If an exception is raised on the Artichoke interpreter, it is returned.
pub fn run(interp: &mut Artichoke, specs: &[String]) -> Result<bool, Exception> {
    interp.def_rb_source_file(b"/src/spec_helper.rb", &b""[..])?;
    interp.def_rb_source_file(b"/src/lib/spec_helper.rb", &b""[..])?;
    interp.def_rb_source_file(
        b"/src/test/spec_runner",
        &include_bytes!("spec_runner.rb")[..],
    )?;
    interp.eval(b"require '/src/test/spec_runner'")?;
    let specs = interp.convert_mut(specs);
    let result = interp
        .top_self()
        .funcall::<bool>("run_specs", &[specs], None)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    #[test]
    // TODO: GH-528 - fix failing tests on Windows.
    #[cfg_attr(target_os = "windows", should_panic)]
    fn mspec_framework_loads() {
        let mut interp = artichoke_backend::interpreter().unwrap();
        super::init(&mut interp).unwrap();
        // should not panic
        assert!(super::run(&mut interp, &[]).unwrap());
    }
}
