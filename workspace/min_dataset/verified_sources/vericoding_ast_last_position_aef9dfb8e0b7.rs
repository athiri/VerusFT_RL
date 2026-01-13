use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
# [verifier :: loop_isolation (false)] fn last_position (a : & [i32] , elem : i32) -> (result : usize) requires a . len () > 0 , exists | k : int | 0 <= k < a . len () && a [k] == elem , ensures 0 <= result < a . len () , forall | i : int | result < i < a . len () ==> a [i] != elem , a [result as int] == elem , { let mut i = a . len () - 1 ; loop invariant 0 <= i < a . len () , forall | j : int | i < j < a . len () ==> a [j] != elem , decreases i { if a [i] == elem { return i ; } if i == 0 { break ; } i -= 1 ; } proof { assert (forall | j : int | 0 <= j < a . len () ==> a [j] != elem) ; assert (exists | k : int | 0 <= k < a . len () && a [k] == elem) ; assert (false) ; } 0 }

} // verus!