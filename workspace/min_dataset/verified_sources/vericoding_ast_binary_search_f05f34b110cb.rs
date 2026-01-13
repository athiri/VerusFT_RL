use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
# [verifier :: loop_isolation (false)] fn binary_search (arr : & [i32] , target : i32) -> (result : Option < usize >) requires forall | i : int , j : int | 0 <= i && i < j && j < arr . len () ==> arr [i] <= arr [j] , ensures match result { Some (idx) => 0 <= idx < arr . len () && arr [idx as int] == target , None => forall | i : int | 0 <= i < arr . len () ==> arr [i] != target , } , { let mut low = 0 ; let mut high = arr . len () ; while low < high invariant low <= high && high <= arr . len () , forall | i : int | 0 <= i && i < low ==> arr [i] < target , forall | i : int | high <= i && i < arr . len () ==> arr [i] > target , decreases high - low , { let mid = low + (high - low) / 2 ; if arr [mid] == target { return Some (mid) ; } else if arr [mid] < target { low = mid + 1 ; } else { high = mid ; } } None }

} // verus!