//! [`MatchData#pre_match`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-pre_match)

use crate::extn::core::matchdata::MatchData;
use crate::extn::prelude::*;

pub fn method(interp: &mut Artichoke, value: &Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::try_from_ruby(interp, value) }?;
    let borrow = data.borrow();
    let pre_match = &borrow.string[0..borrow.region.start];
    Ok(interp.convert_mut(pre_match))
}
