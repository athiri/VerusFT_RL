use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
# [verifier :: loop_isolation (false)] fn last_position (a : & [i32] , elem : i32) -> (result : usize) requires 0 < a . len () < 100_000 , exists | i : int | 0 <= i < a . len () && a [i] == elem , ensures 0 <= result < a . len () , forall | i : int | result < i < a . len () ==> a [i] != elem , a [result as int] == elem , { let mut last_idx = 0 ; let mut i = 0 ; while i < a . len () && a [i] != elem invariant 0 <= i <= a . len () , forall | j : int | 0 <= j < i ==> a [j] != elem , decreases a . len () - i { i += 1 ; } last_idx = i ; i += 1 ; while i < a . len () invariant 0 <= last_idx < a . len () , last_idx < i <= a . len () , a [last_idx as int] == elem , forall | j : int | last_idx < j < i ==> a [j] != elem , decreases a . len () - i { if a [i] == elem { last_idx = i ; } i += 1 ; } last_idx }

} // verus!