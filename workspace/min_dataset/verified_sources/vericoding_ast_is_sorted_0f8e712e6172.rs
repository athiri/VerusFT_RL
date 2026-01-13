use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn is_sorted (arr : & Vec < i32 >) -> (is_sorted : bool) requires arr . len () > 0 , ensures is_sorted == (forall | i : int , j : int | 0 <= i < j < arr . len () ==> (arr [i] <= arr [j])) , { let mut k = 0 ; while k < arr . len () - 1 invariant 0 <= k <= arr . len () - 1 , forall | i : int , j : int | 0 <= i < j < k + 1 ==> arr [i] <= arr [j] , decreases arr . len () - 1 - k { if arr [k] > arr [k + 1] { return false ; } k += 1 ; } true }

} // verus!