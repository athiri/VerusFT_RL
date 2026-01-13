use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn min_second_value_first (arr : & Vec < Vec < i32 > >) -> (first_of_min_second : i32) requires arr . len () > 0 , forall | i : int | 0 <= i < arr . len () ==> # [trigger] arr [i] . len () >= 2 , ensures exists | i : int | 0 <= i < arr . len () && first_of_min_second == # [trigger] arr [i] [0] && (forall | j : int | 0 <= j < arr . len () ==> (arr [i] [1] <= # [trigger] arr [j] [1])) , { assert (0 < arr . len ()) ; assert (arr [0] . len () >= 2) ; let mut min_second = arr [0] [1] ; let mut result_first = arr [0] [0] ; let mut idx = 0 ; while idx < arr . len () invariant 0 <= idx <= arr . len () , arr . len () > 0 , forall | i : int | 0 <= i < arr . len () ==> # [trigger] arr [i] . len () >= 2 , forall | k : int | 0 <= k < idx ==> min_second <= # [trigger] arr [k] [1] , exists | k : int | 0 <= k < arr . len () && result_first == # [trigger] arr [k] [0] && arr [k] [1] == min_second , forall | k : int | 0 <= k < idx ==> min_second <= # [trigger] arr [k] [1] , decreases arr . len () - idx , { assert (idx < arr . len ()) ; assert (arr [idx as int] . len () >= 2) ; if arr [idx] [1] < min_second { min_second = arr [idx] [1] ; result_first = arr [idx] [0] ; } idx = idx + 1 ; } assert (forall | j : int | 0 <= j < arr . len () ==> min_second <= # [trigger] arr [j] [1]) ; result_first }

} // verus!