use vstd::prelude::*;
# [allow (unused_imports)] use vstd :: prelude :: * ;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn find_max (nums : Vec < i32 >) -> (ret : i32) requires nums . len () > 0 , ensures forall | i : int | 0 <= i < nums @ . len () ==> nums @ [i] <= ret , exists | i : int | 0 <= i < nums @ . len () ==> nums @ [i] == ret , { let mut max_val = nums [0] ; let mut max_idx = 0 ; for idx in 1 .. nums . len () invariant 0 <= max_idx < nums @ . len () , max_val == nums @ [max_idx as int] , forall | i : int | 0 <= i < idx ==> nums @ [i] <= max_val , { if nums [idx] > max_val { max_val = nums [idx] ; max_idx = idx ; } } max_val }

} // verus!