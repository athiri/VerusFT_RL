use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
# [verifier :: loop_isolation (false)] fn max (a : & [i32]) -> (x : usize) requires a . len () > 0 , ensures 0 <= x < a . len () , forall | k : int | 0 <= k < a . len () ==> a [k] <= a [x as int] , { let mut max_idx : usize = 0 ; let mut i : usize = 1 ; while i < a . len () invariant 0 <= max_idx < a . len () , 1 <= i <= a . len () , forall | k : int | 0 <= k < i ==> a [k] <= a [max_idx as int] , decreases a . len () - i , { if a [i] > a [max_idx] { max_idx = i ; } i = i + 1 ; } max_idx }

} // verus!