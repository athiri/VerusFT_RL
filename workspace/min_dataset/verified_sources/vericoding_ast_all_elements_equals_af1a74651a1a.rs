use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn all_elements_equals (arr : & Vec < i32 > , element : i32) -> (result : bool) ensures result == (forall | i : int | 0 <= i < arr . len () ==> (arr [i] == element)) , { for i in 0 .. arr . len () invariant forall | j : int | 0 <= j < i ==> arr [j] == element { if arr [i] != element { return false ; } } true }

} // verus!