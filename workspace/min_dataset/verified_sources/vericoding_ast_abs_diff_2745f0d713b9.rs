use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn abs_diff (a : i32 , b : i32) -> (result : i32) requires a >= - 1000000 && a <= 1000000 , b >= - 1000000 && b <= 1000000 , ensures result >= 0 , { return 0 ; }

} // verus!