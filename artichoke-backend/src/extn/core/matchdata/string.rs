//! [`MatchData#string`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-string)

use crate::extn::core::matchdata::MatchData;
use crate::extn::prelude::*;

pub fn method(interp: &mut Artichoke, value: &Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::try_from_ruby(interp, value) }?;
    let mut result = interp.convert_mut(data.borrow().string.as_slice());
    result
        .freeze()
        .map_err(|_| Fatal::new(interp, "Unable to freeze MatchData#string result"))?;
    Ok(result)
}
