use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn contains_k (arr : & Vec < i32 > , k : i32) -> (result : bool) ensures result == (exists | i : int | 0 <= i < arr . len () && (arr [i] == k)) , { for idx in 0 .. arr . len () invariant forall | i : int | 0 <= i < idx ==> arr [i] != k , { if arr [idx] == k { return true ; } } false }

} // verus!