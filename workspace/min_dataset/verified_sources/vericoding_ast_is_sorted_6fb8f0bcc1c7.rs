use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
# [verifier :: loop_isolation (false)] fn is_sorted (lst : & [i32]) -> (result : bool) requires lst . len () >= 1 , ensures result <== forall | i : int , j : int | 0 <= i && i < j && j < lst . len () ==> lst [i] <= lst [j] , ! result ==> exists | i : int , j : int | 0 <= i && i < j && j < lst . len () && lst [i] > lst [j] , { for k in 1 .. lst . len () invariant forall | i : int , j : int | 0 <= i && i < j && j < k ==> lst [i] <= lst [j] , { if lst [k - 1] > lst [k] { return false ; } } true }

} // verus!