use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
pub fn myfun (a : & mut Vec < i32 > , b : & mut Vec < i32 > , sum : & mut Vec < i32 > , N : i32) requires N > 0 , N <= MAX / 2 - 1 , old (a) . len () == N , old (b) . len () == N , old (sum) . len () == 1 , ensures forall | k : int | 0 <= k < N ==> a [k] == 2 * N + 1 , { let mut i = 0 ; let value = 2 * N + 1 ; while i < N invariant 0 <= i <= N , a . len () == N , value == 2 * N + 1 , forall | k : int | 0 <= k < i ==> a [k as int] == value , decreases N - i { a . set (i as usize , value) ; i = i + 1 ; } }

} // verus!