use crate::extn::core::string::{self, trampoline};
use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.0.borrow().class_spec::<string::String>().is_some() {
        return Ok(());
    }
    let spec = class::Spec::new("String", None, None)?;
    class::Builder::for_spec(interp, &spec)
        .add_method("ord", artichoke_string_ord, sys::mrb_args_none())?
        .add_method("scan", artichoke_string_scan, sys::mrb_args_req(1))?
        .define()?;
    interp.0.borrow_mut().def_class::<string::String>(spec);
    let _ = interp.eval(&include_bytes!("string.rb")[..])?;
    trace!("Patched String onto interpreter");
    Ok(())
}

unsafe extern "C" fn artichoke_string_ord(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let result = trampoline::ord(&mut interp, value);
    match result {
        Ok(value) => value.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}

unsafe extern "C" fn artichoke_string_scan(
    mrb: *mut sys::mrb_state,
    slf: sys::mrb_value,
) -> sys::mrb_value {
    let (pattern, block) = mrb_get_args!(mrb, required = 1, &block);
    let mut interp = unwrap_interpreter!(mrb);
    let value = Value::new(&interp, slf);
    let pattern = Value::new(&interp, pattern);
    let result = trampoline::scan(&mut interp, value, pattern, block);
    match result {
        Ok(result) => result.inner(),
        Err(exception) => exception::raise(interp, exception),
    }
}
