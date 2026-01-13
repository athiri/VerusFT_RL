use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn m1 (x : i32 , y : i32) -> (z : i32) requires 0 < x < y , ensures z >= 0 && z <= y && z != x , { 0 }

} // verus!