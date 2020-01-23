use std::collections::HashMap;

use crate::extn::core::env::Env;
use crate::extn::prelude::*;
use crate::fs;

#[derive(Debug, Default, Clone, Copy)]
pub struct System;

impl System {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Env for System {
    fn get(&self, interp: &Artichoke, name: &[u8]) -> Result<Option<Vec<u8>>, Exception> {
        // Per Rust docs for `std::env::set_var` and `std::env::remove_var`:
        // https://doc.rust-lang.org/std/env/fn.set_var.html
        // https://doc.rust-lang.org/std/env/fn.remove_var.html
        //
        // This function may panic if key is empty, contains an ASCII equals
        // sign '=' or the NUL character '\0', or when the value contains the
        // NUL character.
        if name.is_empty() {
            // MRI accepts empty names on get and should always return `nil`
            // since empty names are invalid at the OS level.
            Ok(None)
        } else if memchr::memchr(b'\0', name).is_some() {
            Err(Exception::from(ArgumentError::new(
                interp,
                "bad environment variable name: contains null byte",
            )))
        } else if memchr::memchr(b'=', name).is_some() {
            // MRI accepts names containing '=' on get and should always return
            // `nil` since these names are invalid at the OS level.
            Ok(None)
        } else {
            let name = fs::bytes_to_osstr(interp, name)?;
            if let Some(value) = std::env::var_os(name) {
                fs::osstr_to_bytes(interp, value.as_os_str()).map(|bytes| Some(bytes.to_vec()))
            } else {
                Ok(None)
            }
        }
    }

    fn put(
        &mut self,
        interp: &Artichoke,
        name: &[u8],
        value: Option<&[u8]>,
    ) -> Result<(), Exception> {
        // Per Rust docs for `std::env::set_var` and `std::env::remove_var`:
        // https://doc.rust-lang.org/std/env/fn.set_var.html
        // https://doc.rust-lang.org/std/env/fn.remove_var.html
        //
        // This function may panic if key is empty, contains an ASCII equals
        // sign '=' or the NUL character '\0', or when the value contains the
        // NUL character.
        if name.is_empty() {
            // TODO: This should raise `Errno::EINVAL`.
            Err(Exception::from(ArgumentError::new(
                interp,
                "Invalid argument - setenv()",
            )))
        } else if memchr::memchr(b'\0', name).is_some() {
            Err(Exception::from(ArgumentError::new(
                interp,
                "bad environment variable name: contains null byte",
            )))
        } else if memchr::memchr(b'=', name).is_some() {
            let mut message = b"Invalid argumen - setenv(".to_vec();
            message.extend(name.to_vec());
            message.push(b')');
            // TODO: This should raise `Errno::EINVAL`.
            Err(Exception::from(ArgumentError::new_raw(interp, message)))
        } else if let Some(value) = value {
            if memchr::memchr(b'\0', value).is_some() {
                Err(Exception::from(ArgumentError::new(
                    interp,
                    "bad environment variable value: contains null byte",
                )))
            } else {
                std::env::set_var(
                    fs::bytes_to_osstr(interp, name)?,
                    fs::bytes_to_osstr(interp, value)?,
                );
                Ok(())
            }
        } else {
            let name = fs::bytes_to_osstr(interp, name)?;
            std::env::remove_var(name);
            Ok(())
        }
    }

    fn as_map(&self, interp: &Artichoke) -> Result<HashMap<Vec<u8>, Vec<u8>>, Exception> {
        let mut map = HashMap::default();
        for (name, value) in std::env::vars_os() {
            let name = fs::osstr_to_bytes(interp, name.as_os_str())?;
            let value = fs::osstr_to_bytes(interp, value.as_os_str())?;
            map.insert(name.to_vec(), value.to_vec());
        }
        Ok(map)
    }
}
