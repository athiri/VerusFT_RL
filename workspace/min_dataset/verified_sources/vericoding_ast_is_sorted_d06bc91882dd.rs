use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
# [verifier :: loop_isolation (false)] fn is_sorted (lst : & [i32]) -> (result : bool) requires lst . len () >= 1 , ensures result <== forall | i : int , j : int | 0 <= i && i < j && j < lst . len () ==> lst [i] <= lst [j] , ! result ==> exists | i : int , j : int | 0 <= i && i < j && j < lst . len () && lst [i] > lst [j] , { let mut i = 0 ; while i < lst . len () - 1 invariant 0 <= i <= lst . len () - 1 , forall | k : int , l : int | 0 <= k && k < l && l < i + 1 ==> lst [k] <= lst [l] , decreases lst . len () - 1 - i , { if lst [i] > lst [i + 1] { return false ; } i += 1 ; } true }

} // verus!