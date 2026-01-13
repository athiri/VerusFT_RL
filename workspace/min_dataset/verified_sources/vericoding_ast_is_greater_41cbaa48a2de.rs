use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn is_greater (arr : & Vec < i32 > , number : i32) -> (result : bool) ensures result == (forall | i : int | 0 <= i < arr . len () ==> number > arr [i]) , { let mut index = 0 ; while index < arr . len () invariant 0 <= index <= arr . len () , forall | i : int | 0 <= i < index ==> number > arr [i] , decreases arr . len () - index , { if number <= arr [index] { return false ; } index += 1 ; } true }

} // verus!