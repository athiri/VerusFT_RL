use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn all_elements_equals (arr : & Vec < i32 > , element : i32) -> (result : bool) ensures result == (forall | i : int | 0 <= i < arr . len () ==> (arr [i] == element)) , { let mut index = 0 ; while index < arr . len () invariant 0 <= index <= arr . len () , forall | i : int | 0 <= i < index ==> arr [i] == element , decreases arr . len () - index { if arr [index] != element { return false ; } index += 1 ; } true }

} // verus!