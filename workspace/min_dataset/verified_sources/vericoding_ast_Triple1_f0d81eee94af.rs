use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn Triple1 (x : i32) -> (r : i32) requires - 715827882 <= x <= 715827882 ensures r == 3 * x { 3 * x }

} // verus!