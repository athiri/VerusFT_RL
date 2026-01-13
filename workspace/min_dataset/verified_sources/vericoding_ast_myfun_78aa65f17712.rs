use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
pub fn myfun (a : & mut Vec < i32 > , sum : & mut Vec < i32 > , N : i32) requires N > 0 , old (a) . len () == N , old (sum) . len () == 1 , N <= 46340 , N * (N + 2) <= MAX , ensures sum [0] == N * (N + 2) , { let result = N * (N + 2) ; sum . set (0 , result) ; assert (sum [0] == result) ; }

} // verus!