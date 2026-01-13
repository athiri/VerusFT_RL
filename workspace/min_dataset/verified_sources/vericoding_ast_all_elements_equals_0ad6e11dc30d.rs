use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn all_elements_equals (arr : & Vec < i32 > , element : i32) -> (result : bool) ensures result == (forall | i : int | 0 <= i < arr . len () ==> (arr [i] == element)) , { let mut i = 0 ; while i < arr . len () invariant 0 <= i <= arr . len () , forall | j : int | 0 <= j < i ==> arr [j] == element , decreases arr . len () - i { if arr [i] != element { return false ; } i += 1 ; } true }

} // verus!