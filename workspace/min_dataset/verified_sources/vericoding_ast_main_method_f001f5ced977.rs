use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn main_method (n : i32 , k : i32) -> (k_out : i32) requires n > 0 , k > n , n <= 100 , k <= 200 , ensures k_out >= 0 , { return 0 ; }

} // verus!