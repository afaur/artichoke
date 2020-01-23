#![deny(missing_docs, warnings, intra_doc_link_resolution_failure)]

//! Rust bindings for mruby, customized for Artichoke.
//!
//! Bindings are based on the
//! [vendored mruby sources](https://github.com/artichoke/mruby) and generated
//! with bindgen.

use std::ffi::CStr;
use std::fmt;

mod args;
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(clippy::all)]
#[allow(clippy::pedantic)]
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/ffi.rs"));
}

#[path = "ffi_tests.rs"]
#[cfg(test)]
mod ffi_tests;

pub use self::args::*;
pub use self::ffi::*;

/// Version metadata `String` for embedded mruby.
#[must_use]
pub fn mruby_version(verbose: bool) -> String {
    if verbose {
        // Using the unchecked function is safe because these values are C constants
        let engine = unsafe { CStr::from_bytes_with_nul_unchecked(MRUBY_RUBY_ENGINE) };
        let version = unsafe { CStr::from_bytes_with_nul_unchecked(MRUBY_RUBY_VERSION) };
        format!(
            "{} {} [{}]",
            engine.to_string_lossy(),
            version.to_string_lossy(),
            env!("CARGO_PKG_VERSION")
        )
    } else {
        env!("CARGO_PKG_VERSION").to_owned()
    }
}

/// Methods to describe an [`mrb_state`].
pub trait DescribeState {
    /// Wraper around [`fmt::Display`] for [`mrb_state`]. Returns Ruby engine
    /// and interpreter version. For example:
    ///
    /// ```text
    /// mruby 2.0
    /// ```
    fn info(&self) -> String;

    /// Wrapper around [`fmt::Debug`] for [`mrb_state`]. Returns Ruby engine,
    /// interpreter version, engine version, and [`mrb_state`] address. For
    /// example:
    ///
    /// ```text
    /// mruby 2.0 (v2.0.1 rev c078758) interpreter at 0x7f85b8800000
    /// ```
    fn debug(&self) -> String;

    /// Returns detailed interpreter version including engine version. For
    /// example:
    ///
    /// ```text
    /// 2.0 (v2.0.1)
    /// ```
    fn version(&self) -> String;
}

impl DescribeState for *mut mrb_state {
    fn info(&self) -> String {
        if self.is_null() {
            "".to_owned()
        } else {
            format!("{}", unsafe { &**self })
        }
    }

    fn debug(&self) -> String {
        if self.is_null() {
            "".to_owned()
        } else {
            format!("{:?}", unsafe { &**self })
        }
    }

    fn version(&self) -> String {
        // Using the unchecked function is safe because these values are C constants
        let version = unsafe { CStr::from_bytes_with_nul_unchecked(MRUBY_RUBY_VERSION) };
        format!(
            "{} (v{}.{}.{})",
            version.to_string_lossy(),
            MRUBY_RELEASE_MAJOR,
            MRUBY_RELEASE_MINOR,
            MRUBY_RELEASE_TEENY,
        )
    }
}

impl DescribeState for &mrb_state {
    fn info(&self) -> String {
        format!("{}", &**self)
    }

    fn debug(&self) -> String {
        format!("{:?}", &**self)
    }

    fn version(&self) -> String {
        // Using the unchecked function is safe because these values are C constants
        let version = unsafe { CStr::from_bytes_with_nul_unchecked(MRUBY_RUBY_VERSION) };
        format!(
            "{} (v{}.{}.{})",
            version.to_string_lossy(),
            MRUBY_RELEASE_MAJOR,
            MRUBY_RELEASE_MINOR,
            MRUBY_RELEASE_TEENY,
        )
    }
}

impl DescribeState for &mut mrb_state {
    fn info(&self) -> String {
        format!("{}", &**self)
    }

    fn debug(&self) -> String {
        format!("{:?}", &**self)
    }

    fn version(&self) -> String {
        // Using the unchecked function is safe because these values are C constants
        let version = unsafe { CStr::from_bytes_with_nul_unchecked(MRUBY_RUBY_VERSION) };
        format!(
            "{} (v{}.{}.{})",
            version.to_string_lossy(),
            MRUBY_RELEASE_MAJOR,
            MRUBY_RELEASE_MINOR,
            MRUBY_RELEASE_TEENY,
        )
    }
}

impl fmt::Debug for mrb_state {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Using the unchecked function is safe because these values are C constants
        let engine = unsafe { CStr::from_bytes_with_nul_unchecked(MRUBY_RUBY_ENGINE) };
        let version = unsafe { CStr::from_bytes_with_nul_unchecked(MRUBY_RUBY_VERSION) };
        write!(
            f,
            "{} {} (v{}.{}.{}) interpreter at {:p}",
            engine.to_string_lossy(),
            version.to_string_lossy(),
            MRUBY_RELEASE_MAJOR,
            MRUBY_RELEASE_MINOR,
            MRUBY_RELEASE_TEENY,
            self
        )
    }
}

impl fmt::Display for mrb_state {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Using the unchecked function is safe because these values are C constants
        let engine = unsafe { CStr::from_bytes_with_nul_unchecked(MRUBY_RUBY_ENGINE) };
        let version = unsafe { CStr::from_bytes_with_nul_unchecked(MRUBY_RUBY_VERSION) };
        write!(
            f,
            "{} {}",
            engine.to_string_lossy(),
            version.to_string_lossy(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::sys::{mrb_close, mrb_open, DescribeState};

    #[test]
    fn interpreter_display() {
        unsafe {
            let mrb = mrb_open();
            assert_eq!(format!("{}", *mrb), "mruby 2.0");
            assert_eq!(mrb.info(), "mruby 2.0");
            mrb_close(mrb);
        }
    }

    #[test]
    fn interpreter_debug() {
        unsafe {
            let mrb = mrb_open();
            assert_eq!(
                format!("{:?}", *mrb),
                format!("mruby 2.0 (v2.0.1) interpreter at {:p}", &*mrb)
            );
            assert_eq!(
                mrb.debug(),
                format!("mruby 2.0 (v2.0.1) interpreter at {:p}", &*mrb)
            );
            mrb_close(mrb);
        }
    }

    #[test]
    fn version() {
        unsafe {
            let mrb = mrb_open();
            assert_eq!(mrb.version(), "2.0 (v2.0.1)");
            mrb_close(mrb);
        }
    }
}
