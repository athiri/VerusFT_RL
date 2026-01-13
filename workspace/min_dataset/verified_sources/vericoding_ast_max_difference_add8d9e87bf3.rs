use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn max_difference (arr : & Vec < i32 >) -> (diff : i32) requires arr . len () > 0 , forall | i : int | 0 <= i < arr . len () ==> MIN / 2 < # [trigger] arr [i] < MAX / 2 , ensures forall | i : int , j : int | 0 <= i < arr . len () && 0 <= j < arr . len () ==> arr [i] - arr [j] <= diff , { let mut max_val = arr [0] ; let mut min_val = arr [0] ; let mut idx = 1 ; while idx < arr . len () invariant 1 <= idx <= arr . len () , forall | k : int | 0 <= k < idx ==> arr [k] <= max_val , forall | k : int | 0 <= k < idx ==> min_val <= arr [k] , exists | k : int | 0 <= k < idx && arr [k] == max_val , exists | k : int | 0 <= k < idx && arr [k] == min_val , decreases arr . len () - idx { if arr [idx] > max_val { max_val = arr [idx] ; } if arr [idx] < min_val { min_val = arr [idx] ; } idx += 1 ; } max_val - min_val }

} // verus!