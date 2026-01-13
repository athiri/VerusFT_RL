use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
pub fn linear_search (a : & Vec < i32 > , e : i32) -> (n : usize) requires exists | i : int | (0 <= i < a . len () as int) && a [i] == e , ensures 0 <= n < a . len () , a [n as int] == e , forall | k : int | (0 <= k < n as int) ==> a [k] != e , { let mut i = 0 ; while i < a . len () invariant i <= a . len () , forall | k : int | (0 <= k < i as int) ==> a [k] != e , exists | j : int | (i as int <= j < a . len () as int) && a [j] == e , decreases a . len () - i , { if a [i] == e { return i ; } i += 1 ; } assert (false) ; 0 }

} // verus!