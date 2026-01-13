use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn contains_k (arr : & Vec < i32 > , k : i32) -> (result : bool) ensures result == (exists | i : int | 0 <= i < arr . len () && (arr [i] == k)) , { let mut i = 0 ; while i < arr . len () invariant forall | j : int | 0 <= j < i ==> arr [j] != k , decreases arr . len () - i { if arr [i] == k { return true ; } i += 1 ; } false }

} // verus!