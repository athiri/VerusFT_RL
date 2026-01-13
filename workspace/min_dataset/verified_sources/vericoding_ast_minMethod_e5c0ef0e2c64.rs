use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn min (a : int , b : int) -> int { if a < b { a } else { b } }
fn minMethod (a : i32 , b : i32) -> (c : i32) ensures c <= a && c <= b , c == a || c == b , c == min (a as int , b as int) , { if a < b { a } else { b } }

} // verus!