use vstd::prelude::*;
# [allow (unused_imports)] use vstd :: prelude :: * ;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn find_max (nums : Vec < i32 >) -> (ret : i32) requires nums . len () > 0 , ensures forall | i : int | 0 <= i < nums @ . len () ==> nums @ [i] <= ret , exists | i : int | 0 <= i < nums @ . len () ==> nums @ [i] == ret , { let mut max_val = nums [0] ; let mut idx = 0 ; while idx < nums . len () invariant 0 <= idx <= nums . len () , nums . len () > 0 , forall | i : int | 0 <= i < idx ==> nums @ [i] <= max_val , exists | i : int | 0 <= i < nums @ . len () && nums @ [i] == max_val , decreases nums . len () - idx { if nums [idx] > max_val { max_val = nums [idx] ; } idx += 1 ; } max_val }

} // verus!