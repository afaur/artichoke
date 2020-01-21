use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    if interp.state().class_spec::<Object>().is_some() {
        return Ok(());
    }
    let spec = class::Spec::new("Object", None, None)?;
    interp.state_mut().def_class::<Object>(spec);
    let _ = interp.eval(&include_bytes!("object.rb")[..])?;
    trace!("Patched Object onto interpreter");
    Ok(())
}

pub struct Object;
