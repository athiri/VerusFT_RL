use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn is_smaller (arr1 : & Vec < i32 > , arr2 : & Vec < i32 >) -> (result : bool) requires arr1 . len () == arr2 . len () , ensures result == (forall | i : int | 0 <= i < arr1 . len () ==> arr1 [i] > arr2 [i]) , { let mut idx = 0 ; while idx < arr1 . len () invariant 0 <= idx <= arr1 . len () , arr1 . len () == arr2 . len () , forall | i : int | 0 <= i < idx ==> arr1 [i as int] > arr2 [i as int] , decreases arr1 . len () - idx , { assert (idx < arr1 . len ()) ; assert (arr1 . len () == arr2 . len ()) ; assert (idx < arr2 . len ()) ; if arr1 [idx] <= arr2 [idx] { assert (! (arr1 [idx as int] > arr2 [idx as int])) ; assert (exists | i : int | 0 <= i < arr1 . len () && ! (arr1 [i] > arr2 [i])) ; return false ; } idx += 1 ; } assert (forall | i : int | 0 <= i < arr1 . len () ==> arr1 [i] > arr2 [i]) ; true }

} // verus!