use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn myfun (a : & mut Vec < i32 > , sum : & mut Vec < i32 > , N : i32) requires N > 0 , old (a) . len () == N , old (sum) . len () == 1 , N < 1000 , ensures forall | k : int | 0 <= k < N ==> a [k] == N + 1 , { let mut i : usize = 0 ; while i < a . len () invariant i <= a . len () , a . len () == N , forall | k : int | 0 <= k < i ==> a [k] == N + 1 , N + 1 <= MAX , decreases a . len () - i { assert (i < a . len ()) ; assert (N + 1 <= MAX) ; a . set (i , N + 1) ; assert (forall | k : int | 0 <= k < i ==> a [k] == N + 1) ; assert (a [i as int] == N + 1) ; i += 1 ; assert (forall | k : int | 0 <= k < i ==> a [k] == N + 1) ; } }

} // verus!