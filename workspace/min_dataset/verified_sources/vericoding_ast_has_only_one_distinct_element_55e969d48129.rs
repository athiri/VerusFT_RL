use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
# [verifier :: loop_isolation (false)] fn has_only_one_distinct_element (a : & [i32]) -> (result : bool) ensures result ==> forall | i : int , j : int | 0 <= i < a . len () && 0 <= j < a . len () ==> a [i] == a [j] , ! result ==> exists | i : int , j : int | 0 <= i < a . len () && 0 <= j < a . len () && a [i] != a [j] , { if a . len () == 0 { return true ; } let first_element = a [0] ; let mut i : usize = 1 ; while i < a . len () invariant 0 <= i <= a . len () , forall | k : int | 0 <= k < i ==> a [k] == first_element , decreases a . len () - i { if a [i] != first_element { return false ; } i += 1 ; } return true ; }

} // verus!