//! [`MatchData#length`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-length)

use std::convert::TryFrom;

use crate::extn::core::matchdata::MatchData;
use crate::extn::prelude::*;

pub fn method(interp: &Artichoke, value: &Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::try_from_ruby(interp, value) }?;
    let borrow = data.borrow();
    let haystack = &borrow.string[borrow.region.start..borrow.region.end];
    let len = borrow.regexp.inner().captures_len(interp, Some(haystack))?;
    let len = Int::try_from(len)
        .map_err(|_| Fatal::new(interp, "MatchData#length does not fit in Integer max"))?;
    Ok(interp.convert(len))
}
