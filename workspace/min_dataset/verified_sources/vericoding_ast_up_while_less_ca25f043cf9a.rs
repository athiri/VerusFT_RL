use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn up_while_less (N : i32) -> (i : i32) requires 0 <= N , ensures i == N , { let mut i = 0 ; while i < N invariant 0 <= i <= N , decreases N - i , { i = i + 1 ; } i }

} // verus!