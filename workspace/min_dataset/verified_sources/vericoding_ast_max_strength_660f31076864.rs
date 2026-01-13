use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn max_strength_precond (nums : Seq < i64 >) -> bool { nums . len () > 0 }
spec fn max_strength_postcond (nums : Seq < i64 > , result : i64) -> bool { nums . len () > 0 && exists | i : int | 0 <= i < nums . len () && nums [i] == result && forall | j : int | 0 <= j < nums . len () ==> nums [j] <= result }
fn max_strength (nums : Vec < i64 >) -> (result : i64) requires max_strength_precond (nums @) ensures max_strength_postcond (nums @ , result) { let mut max_val = nums [0] ; let mut i = 1 ; while i < nums . len () invariant 0 <= i <= nums . len () , exists | k : int | 0 <= k < i && nums @ [k] == max_val , forall | j : int | 0 <= j < i ==> nums @ [j] <= max_val decreases nums . len () - i { if nums [i] > max_val { max_val = nums [i] ; } i += 1 ; } max_val }

} // verus!