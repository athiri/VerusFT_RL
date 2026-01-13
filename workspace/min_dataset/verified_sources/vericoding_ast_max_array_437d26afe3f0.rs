use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn max_array (nums : & [i32]) -> (idx : usize) requires nums . len () >= 1 , ensures 0 <= idx && idx < nums . len () , forall | i : int | 0 <= i && i < nums . len () ==> nums [i] <= nums [idx as int] , { let mut max_idx = 0 ; let mut i = 1 ; while i < nums . len () invariant 0 <= max_idx < nums . len () , 1 <= i <= nums . len () , forall | j : int | 0 <= j < i ==> nums [j] <= nums [max_idx as int] , decreases nums . len () - i , { if nums [i] > nums [max_idx] { max_idx = i ; } i += 1 ; } max_idx }

} // verus!