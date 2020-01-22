use artichoke_core::value::Value as _;
use std::convert::TryFrom;
use std::mem;

use crate::extn::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum ElementReference {
    Empty,
    Index(Int),
    StartLen(Int, usize),
}

pub fn element_reference(
    interp: &Artichoke,
    elem: Value,
    len: Option<Value>,
    ary_len: usize,
) -> Result<ElementReference, Exception> {
    if let Some(len) = len {
        let start = if let Ok(start) = elem.try_into::<Int>(interp) {
            start
        } else if let Ok(start) = elem.funcall::<Int>("to_int", &[], None) {
            start
        } else {
            let elem_type_name = elem.pretty_name();
            return Err(Exception::from(TypeError::new(
                interp,
                format!("no implicit conversion of {} into Integer", elem_type_name),
            )));
        };
        let len = if let Ok(len) = len.try_into::<Int>(interp) {
            len
        } else if let Ok(len) = len.funcall::<Int>("to_int", &[], None) {
            len
        } else {
            let len_type_name = len.pretty_name();
            return Err(Exception::from(TypeError::new(
                interp,
                format!("no implicit conversion of {} into Integer", len_type_name),
            )));
        };
        if let Ok(len) = usize::try_from(len) {
            Ok(ElementReference::StartLen(start, len))
        } else {
            Ok(ElementReference::Empty)
        }
    } else {
        let name = elem.pretty_name();
        if let Ok(index) = elem.try_into::<Int>(interp) {
            Ok(ElementReference::Index(index))
        } else if let Ok(index) = elem.funcall::<Int>("to_int", &[], None) {
            Ok(ElementReference::Index(index))
        } else {
            let rangelen = Int::try_from(ary_len)
                .map_err(|_| Fatal::new(interp, "Range length exceeds Integer max"))?;
            match unsafe { is_range(interp, &elem, rangelen) } {
                Ok(Some((start, len))) => Ok(ElementReference::StartLen(start, len)),
                Ok(None) => Ok(ElementReference::Empty),
                Err(_) => Err(Exception::from(TypeError::new(
                    interp,
                    format!("no implicit conversion of {} into Integer", name),
                ))),
            }
        }
    }
}

pub fn element_assignment(
    interp: &Artichoke,
    first: Value,
    second: Value,
    third: Option<Value>,
    len: usize,
) -> Result<(usize, Option<usize>, Value), Exception> {
    if let Some(elem) = third {
        let start = first;
        let start_type_name = start.pretty_name();
        let start = if let Ok(start) = start.try_into::<Int>(interp) {
            start
        } else if let Ok(start) = start.funcall::<Int>("to_int", &[], None) {
            start
        } else {
            return Err(Exception::from(TypeError::new(
                interp,
                format!("no implicit conversion of {} into Integer", start_type_name),
            )));
        };
        let start = if let Ok(start) = usize::try_from(start) {
            start
        } else {
            let start = usize::try_from(start)
                .map_err(|_| Fatal::new(interp, "Positive Int must be usize"))?;
            if start < len {
                len - start
            } else {
                return Err(Exception::from(IndexError::new(
                    interp,
                    format!("index {} too small for array; minimum: -{}", start, len),
                )));
            }
        };
        let len = second;
        let len_type_name = len.pretty_name();
        let len = if let Ok(len) = len.try_into::<Int>(interp) {
            len
        } else if let Ok(len) = len.funcall::<Int>("to_int", &[], None) {
            len
        } else {
            return Err(Exception::from(TypeError::new(
                interp,
                format!("no implicit conversion of {} into Integer", len_type_name),
            )));
        };
        if let Ok(len) = usize::try_from(len) {
            Ok((start, Some(len), elem))
        } else {
            Err(Exception::from(IndexError::new(
                interp,
                format!("negative length ({})", len),
            )))
        }
    } else if let Ok(index) = first.try_into::<Int>(interp) {
        if let Ok(index) = usize::try_from(index) {
            Ok((index, None, second))
        } else {
            let index = usize::try_from(-index)
                .map_err(|_| Fatal::new(interp, "Positive Int must be usize"))?;
            if index < len {
                Ok((len - index, None, second))
            } else {
                Err(Exception::from(IndexError::new(
                    interp,
                    format!("index {} too small for array; minimum: -{}", index, len),
                )))
            }
        }
    } else if let Ok(index) = first.funcall::<Int>("to_int", &[], None) {
        if let Ok(index) = usize::try_from(index) {
            Ok((index, None, second))
        } else {
            let index = usize::try_from(-index)
                .map_err(|_| Fatal::new(interp, "Positive Int must be usize"))?;
            if index < len {
                Ok((len - index, None, second))
            } else {
                Err(Exception::from(IndexError::new(
                    interp,
                    format!("index {} too small for array; minimum: -{}", index, len),
                )))
            }
        }
    } else {
        let rangelen = Int::try_from(len)
            .map_err(|_| Fatal::new(interp, "Range length exceeds Integer max"))?;
        match unsafe { is_range(interp, &first, rangelen) } {
            Ok(Some((start, len))) => {
                if let Ok(start) = usize::try_from(start) {
                    Ok((start, Some(len), second))
                } else {
                    Ok((0, Some(len), second))
                }
            }
            Ok(None) => {
                let start = if let Ok(start) = first.funcall::<Value>("begin", &[], None) {
                    start
                } else {
                    return Err(Exception::from(Fatal::new(
                        interp,
                        "Unable to extract first from Range",
                    )));
                };
                let start = if let Ok(start) = start.try_into::<Int>(interp) {
                    start
                } else if let Ok(start) = start.funcall::<Int>("to_int", &[], None) {
                    start
                } else {
                    return Err(Exception::from(TypeError::new(
                        interp,
                        format!(
                            "no implicit conversion of {} into Integer",
                            start.pretty_name()
                        ),
                    )));
                };
                let end = if let Ok(end) = first.funcall::<Value>("last", &[], None) {
                    end
                } else {
                    return Err(Exception::from(Fatal::new(
                        interp,
                        "Unable to extract first from Range",
                    )));
                };
                let end = if let Ok(end) = end.try_into::<Int>(interp) {
                    end
                } else if let Ok(end) = end.funcall::<Int>("to_int", &[], None) {
                    end
                } else {
                    return Err(Exception::from(TypeError::new(
                        interp,
                        format!(
                            "no implicit conversion of {} into Integer",
                            end.pretty_name()
                        ),
                    )));
                };
                if start + (end - start) < 0 {
                    return Err(Exception::from(RangeError::new(
                        interp,
                        format!("{}..{} out of range", start, end),
                    )));
                }
                match (usize::try_from(start), usize::try_from(end)) {
                    (Ok(start), Ok(end)) => {
                        if end > start {
                            Ok((start, Some(end - start), second))
                        } else {
                            Ok((start, None, second))
                        }
                    }
                    (Err(_), Ok(end)) => {
                        let start = usize::try_from(start)
                            .map_err(|_| Fatal::new(interp, "Positive Int must be usize"))?;
                        if start < len {
                            let start = len - start;
                            if end > start {
                                Ok((start, Some(end - start), second))
                            } else {
                                Ok((start, None, second))
                            }
                        } else {
                            Err(Exception::from(IndexError::new(
                                interp,
                                format!("index {} too small for array; minimum: -{}", start, len),
                            )))
                        }
                    }
                    (Ok(start), Err(_)) => Ok((start, None, second)),
                    (Err(_), Err(_)) => Err(Exception::from(IndexError::new(
                        interp,
                        format!("index {} too small for array; minimum: -{}", start, len),
                    ))),
                }
            }
            Err(_) => {
                let index_type_name = first.pretty_name();
                Err(Exception::from(TypeError::new(
                    interp,
                    format!("no implicit conversion of {} into Integer", index_type_name),
                )))
            }
        }
    }
}

unsafe fn is_range(
    interp: &Artichoke,
    range: &Value,
    length: Int,
) -> Result<Option<(Int, usize)>, Exception> {
    let mut start = mem::MaybeUninit::<sys::mrb_int>::uninit();
    let mut len = mem::MaybeUninit::<sys::mrb_int>::uninit();
    let mrb = interp.mrb_mut();
    // `mrb_range_beg_len` can raise.
    // TODO: Wrap this in a call to `mrb_protect`.
    let check_range = sys::mrb_range_beg_len(
        mrb,
        range.inner(),
        start.as_mut_ptr(),
        len.as_mut_ptr(),
        length,
        0_u8,
    );
    let start = start.assume_init();
    let len = len.assume_init();
    if check_range == sys::mrb_range_beg_len::MRB_RANGE_OK {
        let len = usize::try_from(len)
            .map_err(|_| TypeError::new(interp, "no implicit conversion into Integer"))?;
        Ok(Some((start, len)))
    } else {
        Ok(None)
    }
}
