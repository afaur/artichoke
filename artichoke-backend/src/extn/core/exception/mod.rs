//! # Ruby Exception Hierarchy
//!
//! The built-in subclasses of
//! [`Exception`](https://ruby-doc.org/core-2.6.3/Exception.html) are:
//!
//! - `NoMemoryError`
//! - `ScriptError`
//!   - `LoadError`
//!   - `NotImplementedError`
//!   - `SyntaxError`
//! - `SecurityError`
//! - `SignalException`
//!   - `Interrupt`
//! - `StandardError` -- default for `rescue`
//!   - `ArgumentError`
//!     - `UncaughtThrowError`
//!   - `EncodingError`
//!   - `FiberError`
//!   - `IOError`
//!     - `EOFError`
//!   - `IndexError`
//!     - `KeyError`
//!     - `StopIteration`
//!   - `LocalJumpError`
//!   - `NameError`
//!     - `NoMethodError`
//!   - `RangeError`
//!     - `FloatDomainError`
//!   - `RegexpError`
//!   - `RuntimeError` -- default for `raise`
//!     - `FrozenError`
//!   - `SystemCallError`
//!     - `Errno::*`
//!   - `ThreadError`
//!   - `TypeError`
//!   - `ZeroDivisionError`
//! - `SystemExit`
//! - `SystemStackError`
//! - `fatal` -- impossible to rescue

use std::borrow::Cow;
use std::error;
use std::fmt;

use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    let borrow = interp.0.borrow();

    let exception_spec = class::Spec::new("Exception", None, None)?;
    class::Builder::for_spec(interp, &exception_spec)
        .with_super_class(None)
        .define()?;

    let nomemory_spec = class::Spec::new("NoMemoryError", None, None)?;
    class::Builder::for_spec(interp, &nomemory_spec)
        .with_super_class(Some(&exception_spec))
        .define()?;

    let script_spec = class::Spec::new("ScriptError", None, None)?;
    class::Builder::for_spec(interp, &script_spec)
        .with_super_class(Some(&exception_spec))
        .define()?;

    let load_spec = class::Spec::new("LoadError", None, None)?;
    class::Builder::for_spec(interp, &load_spec)
        .with_super_class(Some(&script_spec))
        .define()?;

    let notimplemented_spec = class::Spec::new("NotImplementedError", None, None)?;
    class::Builder::for_spec(interp, &notimplemented_spec)
        .with_super_class(Some(&script_spec))
        .define()?;

    let syntax_spec = class::Spec::new("SyntaxError", None, None)?;
    class::Builder::for_spec(interp, &syntax_spec)
        .with_super_class(Some(&script_spec))
        .define()?;

    let security_spec = class::Spec::new("SecurityError", None, None)?;
    class::Builder::for_spec(interp, &security_spec)
        .with_super_class(Some(&exception_spec))
        .define()?;

    let signal_spec = class::Spec::new("SignalException", None, None)?;
    class::Builder::for_spec(interp, &signal_spec)
        .with_super_class(Some(&exception_spec))
        .define()?;

    let interrupt_spec = class::Spec::new("Interrupt", None, None)?;
    class::Builder::for_spec(interp, &interrupt_spec)
        .with_super_class(Some(&signal_spec))
        .define()?;

    // Default for `rescue`.
    let standard_spec = class::Spec::new("StandardError", None, None)?;
    class::Builder::for_spec(interp, &standard_spec)
        .with_super_class(Some(&exception_spec))
        .define()?;

    let argument_spec = class::Spec::new("ArgumentError", None, None)?;
    class::Builder::for_spec(interp, &argument_spec)
        .with_super_class(Some(&standard_spec))
        .define()?;

    let uncaughthrow_spec = class::Spec::new("UncaughtThrowError", None, None)?;
    class::Builder::for_spec(interp, &uncaughthrow_spec)
        .with_super_class(Some(&argument_spec))
        .define()?;

    let encoding_spec = class::Spec::new("EncodingError", None, None)?;
    class::Builder::for_spec(interp, &encoding_spec)
        .with_super_class(Some(&standard_spec))
        .define()?;

    let fiber_spec = class::Spec::new("FiberError", None, None)?;
    class::Builder::for_spec(interp, &fiber_spec)
        .with_super_class(Some(&standard_spec))
        .define()?;

    let io_spec = class::Spec::new("IOError", None, None)?;
    class::Builder::for_spec(interp, &io_spec)
        .with_super_class(Some(&standard_spec))
        .define()?;

    let eof_spec = class::Spec::new("EOFError", None, None)?;
    class::Builder::for_spec(interp, &eof_spec)
        .with_super_class(Some(&io_spec))
        .define()?;

    let index_spec = class::Spec::new("IndexError", None, None)?;
    class::Builder::for_spec(interp, &index_spec)
        .with_super_class(Some(&standard_spec))
        .define()?;

    let key_spec = class::Spec::new("KeyError", None, None)?;
    class::Builder::for_spec(interp, &key_spec)
        .with_super_class(Some(&index_spec))
        .define()?;

    let stopiteration_spec = class::Spec::new("StopIteration", None, None)?;
    class::Builder::for_spec(interp, &stopiteration_spec)
        .with_super_class(Some(&index_spec))
        .define()?;

    let localjump_spec = class::Spec::new("LocalJumpError", None, None)?;
    class::Builder::for_spec(interp, &localjump_spec)
        .with_super_class(Some(&standard_spec))
        .define()?;

    let name_spec = class::Spec::new("NameError", None, None)?;
    class::Builder::for_spec(interp, &name_spec)
        .with_super_class(Some(&standard_spec))
        .define()?;

    let nomethod_spec = class::Spec::new("NoMethodError", None, None)?;
    class::Builder::for_spec(interp, &nomethod_spec)
        .with_super_class(Some(&name_spec))
        .define()?;

    let range_spec = class::Spec::new("RangeError", None, None)?;
    class::Builder::for_spec(interp, &range_spec)
        .with_super_class(Some(&standard_spec))
        .define()?;

    let floatdomain_spec = class::Spec::new("FloatDomainError", None, None)?;
    class::Builder::for_spec(interp, &floatdomain_spec)
        .with_super_class(Some(&range_spec))
        .define()?;

    let regexp_spec = class::Spec::new("RegexpError", None, None)?;
    class::Builder::for_spec(interp, &regexp_spec)
        .with_super_class(Some(&standard_spec))
        .define()?;

    // Default `Exception` type for `raise`.
    let runtime_spec = class::Spec::new("RuntimeError", None, None)?;
    class::Builder::for_spec(interp, &runtime_spec)
        .with_super_class(Some(&standard_spec))
        .define()?;

    let frozen_spec = class::Spec::new("FrozenError", None, None)?;
    class::Builder::for_spec(interp, &frozen_spec)
        .with_super_class(Some(&runtime_spec))
        .define()?;

    let systemcall_spec = class::Spec::new("SystemCallError", None, None)?;
    class::Builder::for_spec(interp, &systemcall_spec)
        .with_super_class(Some(&standard_spec))
        .define()?;

    let thread_spec = class::Spec::new("ThreadError", None, None)?;
    class::Builder::for_spec(interp, &thread_spec)
        .with_super_class(Some(&standard_spec))
        .define()?;

    let type_spec = class::Spec::new("TypeError", None, None)?;
    class::Builder::for_spec(interp, &type_spec)
        .with_super_class(Some(&standard_spec))
        .define()?;

    let zerodivision_spec = class::Spec::new("ZeroDivisionError", None, None)?;
    class::Builder::for_spec(interp, &zerodivision_spec)
        .with_super_class(Some(&standard_spec))
        .define()?;

    let systemexit_spec = class::Spec::new("SystemExit", None, None)?;
    class::Builder::for_spec(interp, &systemexit_spec)
        .with_super_class(Some(&exception_spec))
        .define()?;

    let systemstack_spec = class::Spec::new("SystemStackError", None, None)?;
    class::Builder::for_spec(interp, &systemstack_spec)
        .with_super_class(Some(&exception_spec))
        .define()?;

    let fatal_spec = class::Spec::new("fatal", None, None)?;
    class::Builder::for_spec(interp, &fatal_spec)
        .with_super_class(Some(&exception_spec))
        .define()?;

    drop(borrow);
    let mut borrow = interp.0.borrow_mut();
    borrow.def_class::<Exception>(exception_spec);
    borrow.def_class::<NoMemoryError>(nomemory_spec);
    borrow.def_class::<ScriptError>(script_spec);
    borrow.def_class::<LoadError>(load_spec);
    borrow.def_class::<NotImplementedError>(notimplemented_spec);
    borrow.def_class::<SyntaxError>(syntax_spec);
    borrow.def_class::<SecurityError>(security_spec);
    borrow.def_class::<SignalException>(signal_spec);
    borrow.def_class::<Interrupt>(interrupt_spec);
    borrow.def_class::<StandardError>(standard_spec);
    borrow.def_class::<ArgumentError>(argument_spec);
    borrow.def_class::<UncaughtThrowError>(uncaughthrow_spec);
    borrow.def_class::<EncodingError>(encoding_spec);
    borrow.def_class::<FiberError>(fiber_spec);
    borrow.def_class::<IOError>(io_spec);
    borrow.def_class::<EOFError>(eof_spec);
    borrow.def_class::<IndexError>(index_spec);
    borrow.def_class::<KeyError>(key_spec);
    borrow.def_class::<StopIteration>(stopiteration_spec);
    borrow.def_class::<LocalJumpError>(localjump_spec);
    borrow.def_class::<NameError>(name_spec);
    borrow.def_class::<NoMethodError>(nomethod_spec);
    borrow.def_class::<RangeError>(range_spec);
    borrow.def_class::<FloatDomainError>(floatdomain_spec);
    borrow.def_class::<RegexpError>(regexp_spec);
    borrow.def_class::<RuntimeError>(runtime_spec);
    borrow.def_class::<FrozenError>(frozen_spec);
    borrow.def_class::<SystemCallError>(systemcall_spec);
    borrow.def_class::<ThreadError>(thread_spec);
    borrow.def_class::<TypeError>(type_spec);
    borrow.def_class::<ZeroDivisionError>(zerodivision_spec);
    borrow.def_class::<SystemExit>(systemexit_spec);
    borrow.def_class::<SystemStackError>(systemstack_spec);
    borrow.def_class::<Fatal>(fatal_spec);
    drop(borrow);

    let _ = interp.eval(&include_bytes!("exception.rb")[..])?;
    trace!("Patched Exception onto interpreter");
    trace!("Patched core exception hierarchy onto interpreter");
    Ok(())
}

macro_rules! ruby_exception_impl {
    ($exception:ident) => {
        #[derive(Debug, Clone)]
        pub struct $exception {
            message: Cow<'static, [u8]>,
        }

        impl $exception {
            pub fn new<S>(interp: &Artichoke, message: S) -> Self
            where
                S: Into<Cow<'static, str>>,
            {
                let _ = interp;
                let message = match message.into() {
                    Cow::Borrowed(s) => Cow::Borrowed(s.as_bytes()),
                    Cow::Owned(s) => Cow::Owned(s.into_bytes()),
                };
                Self { message }
            }

            pub fn new_raw<S>(interp: &Artichoke, message: S) -> Self
            where
                S: Into<Cow<'static, [u8]>>,
            {
                let _ = interp;
                Self {
                    message: message.into(),
                }
            }
        }

        #[allow(clippy::use_self)]
        impl From<$exception> for exception::Exception {
            fn from(exception: $exception) -> exception::Exception {
                exception::Exception::from(Box::<dyn RubyException>::from(exception))
            }
        }

        #[allow(clippy::use_self)]
        impl From<Box<$exception>> for exception::Exception {
            fn from(exception: Box<$exception>) -> exception::Exception {
                exception::Exception::from(Box::<dyn RubyException>::from(exception))
            }
        }

        #[allow(clippy::use_self)]
        impl From<$exception> for Box<dyn RubyException> {
            fn from(exception: $exception) -> Box<dyn RubyException> {
                Box::new(exception)
            }
        }

        #[allow(clippy::use_self)]
        impl From<Box<$exception>> for Box<dyn RubyException> {
            fn from(exception: Box<$exception>) -> Box<dyn RubyException> {
                exception
            }
        }

        impl RubyException for $exception {
            fn message(&self) -> &[u8] {
                self.message.as_ref()
            }

            fn name(&self) -> String {
                String::from(stringify!($exception))
            }

            fn vm_backtrace(&self, interp: &mut Artichoke) -> Option<Vec<Vec<u8>>> {
                let _ = interp;
                None
            }

            fn as_mrb_value(&self, interp: &mut Artichoke) -> Option<sys::mrb_value> {
                let message = interp.convert_mut(self.message());
                let borrow = interp.0.borrow();
                let spec = borrow.class_spec::<Self>()?;
                let value = spec.new_instance(interp, &[message])?;
                Some(value.inner())
            }
        }

        impl fmt::Display for $exception {
            fn fmt(&self, mut f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let classname = self.name();
                write!(f, "{} (", classname)?;
                string::format_unicode_debug_into(&mut f, self.message())
                    .map_err(string::WriteError::into_inner)?;
                write!(f, ")")
            }
        }

        impl error::Error for $exception {}
    };
}

ruby_exception_impl!(Exception);
ruby_exception_impl!(NoMemoryError);
ruby_exception_impl!(ScriptError);
ruby_exception_impl!(LoadError);
ruby_exception_impl!(NotImplementedError);
ruby_exception_impl!(SyntaxError);
ruby_exception_impl!(SecurityError);
ruby_exception_impl!(SignalException);
ruby_exception_impl!(Interrupt);
// Default for `rescue`.
ruby_exception_impl!(StandardError);
ruby_exception_impl!(ArgumentError);
ruby_exception_impl!(UncaughtThrowError);
ruby_exception_impl!(EncodingError);
ruby_exception_impl!(FiberError);
ruby_exception_impl!(IOError);
ruby_exception_impl!(EOFError);
ruby_exception_impl!(IndexError);
ruby_exception_impl!(KeyError);
ruby_exception_impl!(StopIteration);
ruby_exception_impl!(LocalJumpError);
ruby_exception_impl!(NameError);
ruby_exception_impl!(NoMethodError);
ruby_exception_impl!(RangeError);
ruby_exception_impl!(FloatDomainError);
ruby_exception_impl!(RegexpError);
// Default `Exception` type for `raise`.
ruby_exception_impl!(RuntimeError);
ruby_exception_impl!(FrozenError);
ruby_exception_impl!(SystemCallError);
// ruby_exception_impl!(Errno::*);
ruby_exception_impl!(ThreadError);
ruby_exception_impl!(TypeError);
ruby_exception_impl!(ZeroDivisionError);
ruby_exception_impl!(SystemExit);
ruby_exception_impl!(SystemStackError);
// Fatal interpreter error. Impossible to rescue.
ruby_exception_impl!(Fatal);

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    struct Run;

    impl Run {
        unsafe extern "C" fn run(mrb: *mut sys::mrb_state, _slf: sys::mrb_value) -> sys::mrb_value {
            let interp = unwrap_interpreter!(mrb);
            let exc = RuntimeError::new(&interp, "something went wrong");
            exception::raise(interp, exc)
        }
    }

    impl File for Run {
        type Artichoke = Artichoke;

        type Error = Exception;

        fn require(interp: &mut Artichoke) -> Result<(), Self::Error> {
            let spec = class::Spec::new("Run", None, None).unwrap();
            class::Builder::for_spec(interp, &spec)
                .add_self_method("run", Self::run, sys::mrb_args_none())?
                .define()?;
            interp.0.borrow_mut().def_class::<Self>(spec);
            Ok(())
        }
    }

    #[test]
    fn raise() {
        let mut interp = crate::interpreter().expect("init");
        Run::require(&mut interp).unwrap();
        let err = interp.eval(b"Run.run").unwrap_err();
        assert_eq!("RuntimeError", err.name().as_str());
        assert_eq!(Vec::from(&b"something went wrong"[..]), err.message());
        assert_eq!(
            Some(vec![Vec::from(&b"(eval):1"[..])]),
            err.vm_backtrace(&mut interp)
        );
    }
}
