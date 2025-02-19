use crate::extn::core::array::Array;
use crate::extn::prelude::*;
use crate::gc::MrbGarbageCollection;

pub fn clear(interp: &Artichoke, ary: Value) -> Result<Value, Exception> {
    if ary.is_frozen() {
        return Err(Exception::from(FrozenError::new(
            interp,
            "can't modify frozen Array",
        )));
    }
    let array = unsafe { Array::try_from_ruby(interp, &ary) }?;
    let mut borrow = array.borrow_mut();
    borrow.clear();
    Ok(ary)
}

pub fn element_reference(
    interp: &Artichoke,
    ary: Value,
    first: Value,
    second: Option<Value>,
) -> Result<Value, Exception> {
    let array = unsafe { Array::try_from_ruby(interp, &ary) }?;
    let borrow = array.borrow();
    borrow.element_reference(interp, first, second)
}

pub fn element_assignment(
    interp: &Artichoke,
    ary: Value,
    first: Value,
    second: Value,
    third: Option<Value>,
) -> Result<Value, Exception> {
    if ary.is_frozen() {
        return Err(Exception::from(FrozenError::new(
            interp,
            "can't modify frozen Array",
        )));
    }
    let array = unsafe { Array::try_from_ruby(interp, &ary) }?;
    // TODO: properly handle self-referential sets.
    if ary == first || ary == second || Some(ary) == third {
        return Ok(interp.convert(None::<Value>));
    }
    let mut borrow = array.borrow_mut();
    let gc_was_enabled = interp.disable_gc();
    let result = borrow.element_assignment(interp, first, second, third);
    if gc_was_enabled {
        interp.enable_gc();
    }
    result
}

pub fn pop(interp: &Artichoke, ary: Value) -> Result<Value, Exception> {
    if ary.is_frozen() {
        return Err(Exception::from(FrozenError::new(
            interp,
            "can't modify frozen Array",
        )));
    }
    let array = unsafe { Array::try_from_ruby(interp, &ary) }?;
    let mut borrow = array.borrow_mut();
    let gc_was_enabled = interp.disable_gc();
    let result = borrow.pop(interp);
    if gc_was_enabled {
        interp.enable_gc();
    }
    result
}

pub fn concat(interp: &Artichoke, ary: Value, other: Option<Value>) -> Result<Value, Exception> {
    if ary.is_frozen() {
        return Err(Exception::from(FrozenError::new(
            interp,
            "can't modify frozen Array",
        )));
    }
    if let Some(other) = other {
        let array = unsafe { Array::try_from_ruby(interp, &ary) }?;
        let mut borrow = array.borrow_mut();
        let gc_was_enabled = interp.disable_gc();
        borrow.concat(interp, other)?;
        if gc_was_enabled {
            interp.enable_gc();
        }
    }
    Ok(ary)
}

pub fn push(interp: &Artichoke, ary: Value, value: Value) -> Result<Value, Exception> {
    if ary.is_frozen() {
        return Err(Exception::from(FrozenError::new(
            interp,
            "can't modify frozen Array",
        )));
    }
    let array = unsafe { Array::try_from_ruby(interp, &ary) }?;
    let idx = array.borrow().len();
    let mut borrow = array.borrow_mut();
    let gc_was_enabled = interp.disable_gc();
    borrow.set(interp, idx, value)?;
    if gc_was_enabled {
        interp.enable_gc();
    }
    Ok(ary)
}

pub fn reverse_bang(interp: &Artichoke, ary: Value) -> Result<Value, Exception> {
    if ary.is_frozen() {
        return Err(Exception::from(FrozenError::new(
            interp,
            "can't modify frozen Array",
        )));
    }
    let array = unsafe { Array::try_from_ruby(interp, &ary) }?;
    let mut borrow = array.borrow_mut();
    let gc_was_enabled = interp.disable_gc();
    borrow.reverse(interp)?;
    if gc_was_enabled {
        interp.enable_gc();
    }
    Ok(ary)
}

pub fn len(interp: &Artichoke, ary: Value) -> Result<usize, Exception> {
    let array = unsafe { Array::try_from_ruby(interp, &ary) }?;
    let borrow = array.borrow();
    Ok(borrow.len())
}

pub fn initialize(
    interp: &mut Artichoke,
    ary: Value,
    first: Option<Value>,
    second: Option<Value>,
    block: Option<Block>,
) -> Result<Value, Exception> {
    Array::initialize(interp, first, second, block, ary)
}

pub fn initialize_copy(interp: &Artichoke, ary: Value, from: Value) -> Result<Value, Exception> {
    let from = unsafe { Array::try_from_ruby(interp, &from) }?;
    let borrow = from.borrow();
    let result = borrow.clone();
    let result = result.try_into_ruby(interp, Some(ary.inner()))?;
    Ok(result)
}
