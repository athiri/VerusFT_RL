use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
# [verifier :: loop_isolation (false)] fn barrier (arr : & [i32] , p : usize) -> (result : bool) ensures result == forall | k : int , l : int | 0 <= k <= p && p < l < arr . len () ==> arr [k] < arr [l] , { if p >= arr . len () || p + 1 >= arr . len () { return true ; } for i in 0 .. (p + 1) invariant forall | k : int , l : int | 0 <= k < i && p < l < arr . len () ==> arr [k as int] < arr [l] , { for j in (p + 1) .. arr . len () invariant forall | k : int , l : int | 0 <= k < i && p < l < arr . len () ==> arr [k as int] < arr [l] , forall | l : int | (p + 1) <= l < j ==> arr [i as int] < arr [l] , { if arr [i] >= arr [j] { return false ; } } } true }

} // verus!