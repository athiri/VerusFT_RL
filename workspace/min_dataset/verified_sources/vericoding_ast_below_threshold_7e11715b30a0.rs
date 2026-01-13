use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn below_threshold (l : & [i32] , t : i32) -> (result : bool) ensures result == forall | i : int | 0 <= i < l . len () ==> l [i] < t , { let mut idx = 0 ; while idx < l . len () invariant 0 <= idx <= l . len () , forall | i : int | 0 <= i < idx ==> l [i] < t , decreases l . len () - idx { if l [idx] >= t { return false ; } idx += 1 ; } true }

} // verus!