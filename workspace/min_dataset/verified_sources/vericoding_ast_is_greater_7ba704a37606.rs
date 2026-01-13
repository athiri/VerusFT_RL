use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn is_greater (arr : & Vec < i32 > , number : i32) -> (result : bool) ensures result == (forall | i : int | 0 <= i < arr . len () ==> number > arr [i]) , { let mut idx = 0 ; while idx < arr . len () invariant 0 <= idx <= arr . len () , forall | i : int | 0 <= i < idx ==> number > arr [i] , decreases arr . len () - idx , { if number <= arr [idx] { return false ; } idx += 1 ; } true }

} // verus!