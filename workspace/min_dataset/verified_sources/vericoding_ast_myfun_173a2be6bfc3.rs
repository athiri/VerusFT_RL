use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
pub fn myfun (a : & mut Vec < i32 > , N : u32) requires N > 0 , old (a) . len () == N , ensures forall | k : int | 0 <= k < N ==> a [k] % 2 == N % 2 , { let mut i = 0 ; while i < N invariant 0 <= i <= N , a . len () == N , forall | k : int | 0 <= k < i ==> a [k] % 2 == N % 2 , decreases N - i , { let value = if N % 2 == 0 { 0i32 } else { 1i32 } ; a . set (i as usize , value) ; assert (a [i as int] % 2 == N % 2) ; i += 1 ; } }

} // verus!