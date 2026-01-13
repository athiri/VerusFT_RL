use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn has_only_one_distinct_element (arr : & Vec < i32 >) -> (result : bool) ensures result == (forall | i : int | 1 <= i < arr @ . len () ==> arr [0] == # [trigger] arr [i]) , { if arr . len () <= 1 { return true ; } let mut j = 1 ; while j < arr . len () invariant 1 <= j <= arr . len () , forall | k : int | 1 <= k < j ==> arr [0] == # [trigger] arr [k] , decreases arr . len () - j { if arr [0] != arr [j] { return false ; } j += 1 ; } true }

} // verus!