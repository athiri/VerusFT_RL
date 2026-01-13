use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
# [verifier :: loop_isolation (false)] fn pairs_sum_to_zero (nums : & [i32] , target : i32) -> (found : bool) requires nums . len () >= 2 , forall | i : int , j : int | 0 <= i < j < nums . len () ==> nums [i] + nums [j] <= MAX && nums [i] + nums [j] >= MIN , ensures found <==> exists | i : int , j : int | 0 <= i < j < nums . len () && nums [i] + nums [j] == target , { let mut i = 0 ; while i < nums . len () - 1 invariant 0 <= i <= nums . len () - 1 , forall | k : int , l : int | 0 <= k < i && k < l < nums . len () ==> nums @ [k] + nums @ [l] != target , decreases nums . len () - 1 - i { let mut j = i + 1 ; while j < nums . len () invariant 0 <= i < nums . len () - 1 , i + 1 <= j <= nums . len () , forall | k : int , l : int | 0 <= k < i && k < l < nums . len () ==> nums @ [k] + nums @ [l] != target , forall | l : int | i < l < j ==> nums @ [i as int] + nums @ [l] != target , decreases nums . len () - j { if nums [i] + nums [j] == target { return true ; } j += 1 ; } i += 1 ; } false }

} // verus!