use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn F () -> (r : i32) ensures r == 29 { 29 }

} // verus!