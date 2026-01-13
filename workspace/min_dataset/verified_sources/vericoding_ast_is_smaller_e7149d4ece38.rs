use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn is_smaller (arr1 : & Vec < i32 > , arr2 : & Vec < i32 >) -> (result : bool) requires arr1 . len () == arr2 . len () , ensures result == (forall | i : int | 0 <= i < arr1 . len () ==> arr1 [i] > arr2 [i]) , { let mut index = 0 ; while index < arr1 . len () invariant 0 <= index <= arr1 . len () , arr1 . len () == arr2 . len () , forall | i : int | 0 <= i < index ==> arr1 [i] > arr2 [i] , decreases arr1 . len () - index , { assert (index < arr2 . len ()) ; if arr1 [index] <= arr2 [index] { return false ; } index += 1 ; } true }

} // verus!