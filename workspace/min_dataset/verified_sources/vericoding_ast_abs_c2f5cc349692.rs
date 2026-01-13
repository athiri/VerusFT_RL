use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn abs (x : i32) -> (result : i32) requires x > MIN , ensures result >= 0 , result == x || result == - x , { if x >= 0 { x } else { - x } }

} // verus!