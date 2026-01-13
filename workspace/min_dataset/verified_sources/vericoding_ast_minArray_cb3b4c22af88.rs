use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn minArray (a : & [i32]) -> (m : i32) requires a . len () > 0 ensures forall | k : int | 0 <= k < a . len () ==> m <= a [k] , exists | k : int | 0 <= k < a . len () && m == a [k] , { let mut min_val = a [0] ; let mut i = 1 ; while i < a . len () invariant 0 < i <= a . len () , exists | k : int | 0 <= k < i && min_val == a [k] , forall | k : int | 0 <= k < i ==> min_val <= a [k] , decreases a . len () - i { if a [i] < min_val { min_val = a [i] ; } i += 1 ; } min_val }

} // verus!