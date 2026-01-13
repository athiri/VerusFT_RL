use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
pub fn myfun (a : & mut Vec < i32 > , b : & mut Vec < i32 > , sum : & mut Vec < i32 > , N : i32) requires N > 0 , old (a) . len () == N , old (b) . len () == N , old (sum) . len () == 1 , N < 1000 , ensures forall | k : int | 0 <= k < N ==> a [k] == N + 1 , { let mut i = 0 ; while i < N invariant 0 <= i <= N , a . len () == N , forall | k : int | 0 <= k < i ==> a [k] == N + 1 , N < 1000 , decreases N - i , { assert (i < N) ; assert (0 <= i < a . len ()) ; a [i as usize] = N + 1 ; assert (forall | k : int | 0 <= k < i ==> a [k] == N + 1) ; assert (a [i as int] == N + 1) ; i = i + 1 ; assert (forall | k : int | 0 <= k < i ==> a [k] == N + 1) ; } }

} // verus!