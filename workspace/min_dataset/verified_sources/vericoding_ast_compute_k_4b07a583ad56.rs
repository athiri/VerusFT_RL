use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn compute_k (n : i32 , k : i32) -> (k_out : i32) requires n > 0 , k >= n + 1 , k >= 0 , n <= 1000000 , k <= 1000000 , ensures k_out >= 0 , { k }

} // verus!