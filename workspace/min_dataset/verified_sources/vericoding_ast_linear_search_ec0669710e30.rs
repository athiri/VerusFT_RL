use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn linear_search (a : & [int] , e : int) -> (n : usize) requires exists | i : int | 0 <= i < a . len () && a [i] == e , ensures 0 <= n < a . len () , a [n as int] == e , forall | k : int | 0 <= k < n ==> a [k] != e , { let mut i = 0 ; while i < a . len () invariant 0 <= i <= a . len () , forall | k : int | 0 <= k < i ==> a [k] != e , exists | j : int | i <= j < a . len () && a [j] == e , decreases a . len () - i { if a [i] == e { return i ; } i += 1 ; } 0 }

} // verus!