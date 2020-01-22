use std::borrow::Cow;
use std::cell::RefCell;
use std::ffi::c_void;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::class;
use crate::convert::RustBackedValue;
use crate::module;
use crate::sys;
use crate::Artichoke;

/// Typedef for an mruby free function for an [`mrb_value`](sys::mrb_value) with
/// `tt` [`MRB_TT_DATA`](sys::mrb_vtype::MRB_TT_DATA).
pub type Free = unsafe extern "C" fn(mrb: *mut sys::mrb_state, data: *mut c_void);

/// A generic implementation of a [`Free`] function for
/// [`mrb_value`](sys::mrb_value)s that store an owned copy of an [`Rc`] smart
/// pointer.
///
/// This function calls [`Rc::from_raw`] on the data pointer and drops the
/// resulting [`Rc`].
///
/// # Safety
///
/// This function assumes that the data pointer is to an
/// [`Rc`]`<`[`RefCell`]`<T>>` created by [`Rc::into_raw`]. This fuction bounds
/// `T` by [`RustBackedValue`] which boxes `T` for the mruby VM like this.
///
/// This function assumes it is called by the mruby VM as a free function for
/// an [`MRB_TT_DATA`](sys::mrb_vtype::MRB_TT_DATA).
pub unsafe extern "C" fn rust_data_free<T: 'static + RustBackedValue>(
    _mrb: *mut sys::mrb_state,
    data: *mut c_void,
) {
    if data.is_null() {
        panic!(
            "Received null pointer in rust_data_free<{}>",
            T::ruby_type_name()
        );
    }
    let data = Rc::from_raw(data as *const RefCell<T>);
    drop(data);
}

#[cfg(test)]
mod free_test {
    use crate::convert::RustBackedValue;

    fn prototype(_func: super::Free) {}

    struct Data(String);

    impl RustBackedValue for Data {
        fn ruby_type_name() -> &'static str {
            "Data"
        }
    }

    #[test]
    fn free_prototype() {
        prototype(super::rust_data_free::<Data>);
    }
}

/// Typedef for a method exposed in the mruby interpreter.
///
/// This function signature is used for all types of mruby methods, including
/// instance methods, class methods, singleton methods, and global methods.
///
/// `slf` is the method receiver, e.g. `s` in the following invocation of
/// `String#start_with?`.
///
/// ```ruby
/// s = 'artichoke crate'
/// s.start_with?('artichoke')
/// ```
///
/// To extract method arguments, use [`mrb_get_args!`] and the supplied
/// interpreter.
pub type Method =
    unsafe extern "C" fn(mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value;

/// Typesafe wrapper for the [`RClass *`](sys::RClass) of the enclosing scope
/// for an mruby `Module` or `Class`.
///
/// In Ruby, classes and modules can be defined inside of another class or
/// module. mruby only supports resolving [`RClass`](sys::RClass) pointers
/// relative to an enclosing scope. This can be the top level with
/// [`mrb_class_get`](sys::mrb_class_get) and
/// [`mrb_module_get`](sys::mrb_module_get) or it can be under another class
/// with [`mrb_class_get_under`](sys::mrb_class_get_under) or module with
/// [`mrb_module_get_under`](sys::mrb_module_get_under).
///
/// Because there is no C API to resolve class and module names directly, each
/// class-like holds a reference to its enclosing scope so it can recursively
/// resolve its enclosing [`RClass *`](sys::RClass).
#[derive(Clone, Debug)]
pub enum EnclosingRubyScope {
    /// Reference to a Ruby `Class` enclosing scope.
    Class {
        /// Shared copy of the underlying [class definition](class::Spec).
        spec: class::Spec,
    },
    /// Reference to a Ruby `Module` enclosing scope.
    Module {
        /// Shared copy of the underlying [module definition](module::Spec).
        spec: module::Spec,
    },
}

impl EnclosingRubyScope {
    /// Factory for [`EnclosingRubyScope::Class`] that clones an `Rc` smart
    /// pointer wrapped [`class::Spec`].
    ///
    /// This function is useful when extracting an enclosing scope from the
    /// class registry.
    #[must_use]
    pub fn class(spec: &class::Spec) -> Self {
        Self::Class { spec: spec.clone() }
    }

    /// Factory for [`EnclosingRubyScope::Module`] that clones an `Rc` smart
    /// pointer wrapped [`module::Spec`].
    ///
    /// This function is useful when extracting an enclosing scope from the
    /// module registry.
    #[must_use]
    pub fn module(spec: &module::Spec) -> Self {
        Self::Module { spec: spec.clone() }
    }

    /// Resolve the [`RClass *`](sys::RClass) of the wrapped class or module.
    ///
    /// Return [`None`] if the class-like has no [`EnclosingRubyScope`].
    ///
    /// The current implemention results in recursive calls to this function
    /// for each enclosing scope.
    pub fn rclass(&self, interp: &mut Artichoke) -> Option<*mut sys::RClass> {
        match self {
            Self::Class { spec } => spec.rclass(interp),
            Self::Module { spec } => spec.rclass(interp),
        }
    }

    /// Get the fully qualified name of the wrapped class or module.
    ///
    /// For example, in the following Ruby code, `C` has an fqname of `A::B::C`.
    ///
    /// ```ruby
    /// module A
    ///   class B
    ///     module C
    ///       CONST = 1
    ///     end
    ///   end
    /// end
    /// ```
    ///
    /// The current implemention results in recursive calls to this function
    /// for each enclosing scope.
    pub fn fqname(&self) -> Cow<'_, str> {
        match self {
            Self::Class { spec } => spec.fqname(),
            Self::Module { spec } => spec.fqname(),
        }
    }
}

impl Eq for EnclosingRubyScope {}

impl PartialEq for EnclosingRubyScope {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Class { spec: this }, Self::Class { spec: other }) => this == other,
            (Self::Module { spec: this }, Self::Module { spec: other }) => this == other,
            _ => false,
        }
    }
}

impl Hash for EnclosingRubyScope {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Class { spec } => spec.hash(state),
            Self::Module { spec } => spec.hash(state),
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::test::prelude::*;

    #[test]
    fn fqname() {
        struct Root; // A
        struct ModuleUnderRoot; // A::B
        struct ClassUnderRoot; // A::C
        struct ClassUnderModule; // A::B::D
        struct ModuleUnderClass; // A::C::E
        struct ClassUnderClass; // A::C::F

        // Setup: define module and class hierarchy
        let interp = crate::interpreter().expect("init");
        let root = module::Spec::new("A", None).unwrap();
        let mod_under_root =
            module::Spec::new("B", Some(EnclosingRubyScope::module(&root))).unwrap();
        let cls_under_root =
            class::Spec::new("C", Some(EnclosingRubyScope::module(&root)), None).unwrap();
        let cls_under_mod =
            class::Spec::new("D", Some(EnclosingRubyScope::module(&mod_under_root)), None).unwrap();
        let mod_under_cls =
            module::Spec::new("E", Some(EnclosingRubyScope::class(&cls_under_root))).unwrap();
        let cls_under_cls =
            class::Spec::new("F", Some(EnclosingRubyScope::class(&cls_under_root)), None).unwrap();
        module::Builder::for_spec(&interp, &root).define().unwrap();
        module::Builder::for_spec(&interp, &mod_under_root)
            .define()
            .unwrap();
        class::Builder::for_spec(&interp, &cls_under_root)
            .define()
            .unwrap();
        class::Builder::for_spec(&interp, &cls_under_mod)
            .define()
            .unwrap();
        module::Builder::for_spec(&interp, &mod_under_cls)
            .define()
            .unwrap();
        class::Builder::for_spec(&interp, &cls_under_cls)
            .define()
            .unwrap();
        interp.0.borrow_mut().def_module::<Root>(root);
        interp
            .0
            .borrow_mut()
            .def_module::<ModuleUnderRoot>(mod_under_root);
        interp
            .0
            .borrow_mut()
            .def_class::<ClassUnderRoot>(cls_under_root);
        interp
            .0
            .borrow_mut()
            .def_class::<ClassUnderModule>(cls_under_mod);
        interp
            .0
            .borrow_mut()
            .def_module::<ModuleUnderClass>(mod_under_cls);
        interp
            .0
            .borrow_mut()
            .def_class::<ClassUnderClass>(cls_under_cls);

        let borrow = interp.0.borrow();
        let root = borrow.module_spec::<Root>().unwrap();
        assert_eq!(root.fqname().as_ref(), "A");
        assert_eq!(&format!("{}", root), "artichoke module spec -- A");
        let mod_under_root = borrow.module_spec::<ModuleUnderRoot>().unwrap();
        assert_eq!(mod_under_root.fqname().as_ref(), "A::B");
        assert_eq!(
            &format!("{}", mod_under_root),
            "artichoke module spec -- A::B"
        );
        let cls_under_root = borrow.class_spec::<ClassUnderRoot>().unwrap();
        assert_eq!(cls_under_root.fqname().as_ref(), "A::C");
        assert_eq!(
            &format!("{}", cls_under_root),
            "artichoke class spec -- A::C"
        );
        let cls_under_mod = borrow.class_spec::<ClassUnderModule>().unwrap();
        assert_eq!(cls_under_mod.fqname().as_ref(), "A::B::D");
        assert_eq!(
            &format!("{}", cls_under_mod),
            "artichoke class spec -- A::B::D"
        );
        let mod_under_cls = borrow.module_spec::<ModuleUnderClass>().unwrap();
        assert_eq!(mod_under_cls.fqname().as_ref(), "A::C::E");
        assert_eq!(
            &format!("{}", mod_under_cls),
            "artichoke module spec -- A::C::E"
        );
        let cls_under_cls = borrow.class_spec::<ClassUnderClass>().unwrap();
        assert_eq!(cls_under_cls.fqname().as_ref(), "A::C::F");
        assert_eq!(
            &format!("{}", cls_under_cls),
            "artichoke class spec -- A::C::F"
        );
    }

    mod functional {
        use crate::test::prelude::*;

        #[test]
        fn define_method() {
            struct Class;
            struct Module;

            extern "C" fn value(_mrb: *mut sys::mrb_state, slf: sys::mrb_value) -> sys::mrb_value {
                unsafe {
                    match slf.tt {
                        sys::mrb_vtype::MRB_TT_CLASS => sys::mrb_sys_fixnum_value(8),
                        sys::mrb_vtype::MRB_TT_MODULE => sys::mrb_sys_fixnum_value(27),
                        sys::mrb_vtype::MRB_TT_OBJECT => sys::mrb_sys_fixnum_value(64),
                        _ => sys::mrb_sys_fixnum_value(125),
                    }
                }
            }
            let interp = crate::interpreter().expect("init");
            let class = class::Spec::new("DefineMethodTestClass", None, None).unwrap();
            class::Builder::for_spec(&interp, &class)
                .add_method("value", value, sys::mrb_args_none())
                .unwrap()
                .add_self_method("value", value, sys::mrb_args_none())
                .unwrap()
                .define()
                .unwrap();
            interp.0.borrow_mut().def_class::<Class>(class);
            let module = module::Spec::new("DefineMethodTestModule", None).unwrap();
            module::Builder::for_spec(&interp, &module)
                .add_method("value", value, sys::mrb_args_none())
                .unwrap()
                .add_self_method("value", value, sys::mrb_args_none())
                .unwrap()
                .define()
                .unwrap();
            interp.0.borrow_mut().def_module::<Module>(module);

            let _ = interp
                .eval(
                    br#"
                    class DynamicTestClass
                        include DefineMethodTestModule
                        extend DefineMethodTestModule
                    end

                    module DynamicTestModule
                        extend DefineMethodTestModule
                    end
                    "#,
                )
                .expect("eval");

            let result = interp
                .eval(b"DefineMethodTestClass.new.value")
                .expect("eval");
            let result = result.try_into::<i64>().expect("convert");
            assert_eq!(result, 64);
            let result = interp.eval(b"DefineMethodTestClass.value").expect("eval");
            let result = result.try_into::<i64>().expect("convert");
            assert_eq!(result, 8);
            let result = interp.eval(b"DefineMethodTestModule.value").expect("eval");
            let result = result.try_into::<i64>().expect("convert");
            assert_eq!(result, 27);
            let result = interp.eval(b"DynamicTestClass.new.value").expect("eval");
            let result = result.try_into::<i64>().expect("convert");
            assert_eq!(result, 64);
            let result = interp.eval(b"DynamicTestClass.value").expect("eval");
            let result = result.try_into::<i64>().expect("convert");
            assert_eq!(result, 8);
            let result = interp.eval(b"DynamicTestModule.value").expect("eval");
            let result = result.try_into::<i64>().expect("convert");
            assert_eq!(result, 27);
        }
    }
}
