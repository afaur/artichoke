use crate::sys;

pub type Float = f64;

// Parameterize Fixnum integer type based on architecture.
#[cfg(not(target_arch = "wasm32"))]
pub type Int = i64;
// wasm32 builds target 32-bit Ruby `Integer`s.
#[cfg(target_arch = "wasm32")]
pub type Int = i32;

pub use crate::core::types::{Ruby, Rust};

// This conversion has to be from mrb_value instead of mrb_vtype to disambiguate
// between `Ruby::Nil` and a false `Ruby::Bool`.
#[allow(non_upper_case_globals)]
#[must_use]
pub fn ruby_from_mrb_value(value: sys::mrb_value) -> Ruby {
    // `nil` is implemented with the `MRB_TT_FALSE` type tag in mruby
    // (since both values are falsy). The difference is that booleans are
    // non-zero `Fixnum`s.
    if unsafe { sys::mrb_sys_value_is_nil(value) } {
        return Ruby::Nil;
    }

    // switch on the type tag in the `mrb_value`
    #[allow(clippy::match_same_arms)] // ignore lint to map to the `mrb_vtype` enum def
    match value.tt {
        sys::mrb_vtype::MRB_TT_FALSE => Ruby::Bool,
        // `MRB_TT_FREE` is a marker type tag that indicates to the mruby
        // VM that an object should be garbage collected.
        sys::mrb_vtype::MRB_TT_FREE => Ruby::Unreachable,
        sys::mrb_vtype::MRB_TT_TRUE => Ruby::Bool,
        sys::mrb_vtype::MRB_TT_FIXNUM => Ruby::Fixnum,
        sys::mrb_vtype::MRB_TT_SYMBOL => Ruby::Symbol,
        // internal use: #undef; should not happen
        sys::mrb_vtype::MRB_TT_UNDEF => Ruby::Unreachable,
        sys::mrb_vtype::MRB_TT_FLOAT => Ruby::Float,
        // `MRB_TT_CPTR` wraps a borrowed `void *` pointer.
        sys::mrb_vtype::MRB_TT_CPTR => Ruby::CPointer,
        sys::mrb_vtype::MRB_TT_OBJECT => Ruby::Object,
        sys::mrb_vtype::MRB_TT_CLASS => Ruby::Class,
        sys::mrb_vtype::MRB_TT_MODULE => Ruby::Module,
        // `MRB_TT_ICLASS` is an internal use type tag meant for holding
        // mixed in modules.
        sys::mrb_vtype::MRB_TT_ICLASS => Ruby::Unreachable,
        // `MRB_TT_SCLASS` represents a singleton class, or a class that is
        // defined anonymously, e.g. `c1` or `c2` below:
        //
        // ```ruby
        // c1 = Class.new {
        //   def foo; :foo; end
        // }
        // c2 = (class <<cls; self; end)
        // ```
        //
        // mruby also uses the term singleton method to refer to methods
        // defined on an object's eigenclass, e.g. `bar` below:
        //
        // ```ruby
        // class Foo; end
        // obj = Foo.new
        // def obj.bar; 'bar'; end
        // ```
        sys::mrb_vtype::MRB_TT_SCLASS => Ruby::SingletonClass,
        sys::mrb_vtype::MRB_TT_PROC => Ruby::Proc,
        sys::mrb_vtype::MRB_TT_ARRAY => Ruby::Array,
        sys::mrb_vtype::MRB_TT_HASH => Ruby::Hash,
        sys::mrb_vtype::MRB_TT_STRING => Ruby::String,
        sys::mrb_vtype::MRB_TT_RANGE => Ruby::Range,
        sys::mrb_vtype::MRB_TT_EXCEPTION => Ruby::Exception,
        // TODO: Implement File, see GH-4.
        sys::mrb_vtype::MRB_TT_FILE => unimplemented!("mruby type file"),
        // ENV is currently implemented as a singleton object in Ruby.
        sys::mrb_vtype::MRB_TT_ENV => unimplemented!("mruby type env"),
        // `MRB_TT_DATA` is a type tag for wrapped C pointers. It is used
        // to indicate that an `mrb_value` has an owned pointer to an
        // external data structure stored in its `value.p` field.
        sys::mrb_vtype::MRB_TT_DATA => Ruby::Data,
        sys::mrb_vtype::MRB_TT_FIBER => Ruby::Fiber,
        // MRB_TT_ISTRUCT is an "inline structure", or a mrb_value that
        // stores data in a char* buffer inside an mrb_value. These
        // mrb_values cannot have a finalizer and cannot have instance
        // variables.
        //
        // See vendor/mruby-*/include/mruby/istruct.h
        sys::mrb_vtype::MRB_TT_ISTRUCT => Ruby::InlineStruct,
        // `MRB_TT_BREAK` is used internally to the mruby VM and appears to
        // have something to do with resuming continuations from Fibers.
        sys::mrb_vtype::MRB_TT_BREAK => Ruby::Unreachable,
        // `MRB_TT_MAXDEFINE` is a marker enum value used by the mruby VM to
        // dynamically check if a type tag is valid using the less than
        // operator. It does not correspond to an instantiated type.
        sys::mrb_vtype::MRB_TT_MAXDEFINE => Ruby::Unreachable,
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use crate::sys;
    use crate::types::{self, Ruby};

    #[test]
    fn nil_type() {
        unsafe {
            let value = sys::mrb_sys_nil_value();
            assert_eq!(types::ruby_from_mrb_value(value), Ruby::Nil);
        }
    }

    #[test]
    fn bool_type() {
        unsafe {
            let value = sys::mrb_sys_false_value();
            assert_eq!(types::ruby_from_mrb_value(value), Ruby::Bool);
            let value = sys::mrb_sys_true_value();
            assert_eq!(types::ruby_from_mrb_value(value), Ruby::Bool);
        }
    }

    #[test]
    fn fixnum_type() {
        unsafe {
            let value = sys::mrb_sys_fixnum_value(17);
            assert_eq!(types::ruby_from_mrb_value(value), Ruby::Fixnum);
        }
    }

    #[test]
    fn string_type() {
        unsafe {
            let mrb = sys::mrb_open();
            let literal = "dinner plate";
            let cstr = CString::new(literal).unwrap();
            let value = sys::mrb_str_new_cstr(mrb, cstr.as_ptr());
            assert_eq!(types::ruby_from_mrb_value(value), Ruby::String);
            sys::mrb_close(mrb);
        }
    }
}
