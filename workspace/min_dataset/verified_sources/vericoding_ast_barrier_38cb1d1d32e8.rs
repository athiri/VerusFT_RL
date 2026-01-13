use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
# [verifier :: loop_isolation (false)] fn barrier (arr : & [i32] , p : usize) -> (result : bool) ensures result == forall | k : int , l : int | 0 <= k <= p && p < l < arr . len () ==> arr [k] < arr [l] , { if p >= arr . len () { return true ; } let mut i = 0 ; while i <= p invariant 0 <= i <= p + 1 , forall | k : int , l : int | 0 <= k < i && p < l < arr . len () ==> arr [k] < arr [l] , decreases p + 1 - i { let mut j = p + 1 ; while j < arr . len () invariant p + 1 <= j <= arr . len () , forall | k : int , l : int | 0 <= k < i && p < l < arr . len () ==> arr [k] < arr [l] , forall | l : int | p < l < j ==> arr [i as int] < arr [l] , decreases arr . len () - j { if arr [i] >= arr [j] { return false ; } j += 1 ; } i += 1 ; } true }

} // verus!