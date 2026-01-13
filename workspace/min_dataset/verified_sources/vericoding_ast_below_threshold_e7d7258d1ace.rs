use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn below_threshold (l : & [i32] , t : i32) -> (result : bool) ensures result == forall | i : int | 0 <= i < l . len () ==> l [i] < t , { for i in 0 .. l . len () invariant forall | j : int | 0 <= j < i ==> l [j] < t , { if l [i] >= t { return false ; } } true }

} // verus!