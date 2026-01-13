use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn is_sorted (arr : & Vec < i32 >) -> (is_sorted : bool) requires arr . len () > 0 , ensures is_sorted == (forall | i : int , j : int | 0 <= i < j < arr . len () ==> (arr [i] <= arr [j])) , { let mut idx = 1 ; while idx < arr . len () invariant 1 <= idx <= arr . len () , forall | i : int , j : int | 0 <= i < j < idx ==> arr [i] <= arr [j] , decreases arr . len () - idx , { if arr [idx - 1] > arr [idx] { return false ; } idx += 1 ; } true }

} // verus!