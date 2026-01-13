use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn max_difference (arr : & Vec < i32 >) -> (diff : i32) requires arr . len () > 0 , forall | i : int | 0 <= i < arr . len () ==> MIN / 2 < # [trigger] arr [i] < MAX / 2 , ensures forall | i : int , j : int | 0 <= i < arr . len () && 0 <= j < arr . len () ==> arr [i] - arr [j] <= diff , { let mut min_val = arr [0] ; let mut max_val = arr [0] ; let mut k = 1 ; while k < arr . len () invariant 1 <= k <= arr . len () , forall | i : int | 0 <= i < k ==> min_val <= arr [i] , forall | i : int | 0 <= i < k ==> arr [i] <= max_val , exists | i : int | 0 <= i < k && arr [i] == min_val , exists | i : int | 0 <= i < k && arr [i] == max_val , decreases arr . len () - k { if arr [k] < min_val { min_val = arr [k] ; } if arr [k] > max_val { max_val = arr [k] ; } k += 1 ; } max_val - min_val }

} // verus!