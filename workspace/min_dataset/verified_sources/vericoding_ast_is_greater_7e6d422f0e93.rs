use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn is_greater (arr : & Vec < i32 > , number : i32) -> (result : bool) ensures result == (forall | i : int | 0 <= i < arr . len () ==> number > arr [i]) , { for i in 0 .. arr . len () invariant forall | j : int | 0 <= j < i ==> number > arr [j] , { if arr [i] >= number { return false ; } } true }

} // verus!