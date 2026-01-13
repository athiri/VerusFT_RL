use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn contains (arr : & Vec < i32 > , key : i32) -> (result : bool) ensures result == (exists | i : int | 0 <= i < arr . len () && (arr [i] == key)) , { for i in 0 .. arr . len () invariant forall | j : int | 0 <= j < i ==> arr [j] != key , { if arr [i] == key { return true ; } } false }

} // verus!