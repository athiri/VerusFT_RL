use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn contains_k (arr : & Vec < i32 > , k : i32) -> (result : bool) ensures result == (exists | i : int | 0 <= i < arr . len () && (arr [i] == k)) , { let mut idx = 0 ; while idx < arr . len () invariant forall | i : int | 0 <= i < idx ==> arr [i] != k , decreases arr . len () - idx , { if arr [idx] == k { return true ; } idx += 1 ; } false }

} // verus!