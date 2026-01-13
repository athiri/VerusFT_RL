use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn down_while_not_equal (N : i32) -> (i : i32) requires 0 <= N , ensures i == 0 , { let mut i = N ; while i != 0 invariant 0 <= i <= N , decreases i , { i = i - 1 ; } i }

} // verus!