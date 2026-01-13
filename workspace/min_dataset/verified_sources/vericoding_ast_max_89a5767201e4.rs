use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn max (a : i32 , b : i32) -> (c : i32) ensures c >= a && c >= b { if a >= b { a } else { b } }

} // verus!