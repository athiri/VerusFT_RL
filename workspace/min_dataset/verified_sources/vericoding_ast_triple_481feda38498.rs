use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn triple (x : i64) -> (r : i64) requires - 1000000 <= x <= 1000000 , ensures r == 3 * x { 3 * x }

} // verus!