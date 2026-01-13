use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn MyMethod (x : i32) -> (y : i32) requires 10 <= x <= 1000 ensures 25 <= y { x + 15 }

} // verus!