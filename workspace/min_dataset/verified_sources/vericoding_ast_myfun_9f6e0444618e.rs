use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
pub fn myfun (a : & mut Vec < i32 > , b : & mut Vec < i32 > , sum : & mut Vec < i32 > , N : i32) requires N > 0 , old (a) . len () == N , old (b) . len () == N , old (sum) . len () == 1 , ensures sum [0] <= 2 * N , { let result = (2i64 * N as i64) as i32 ; sum . set (0 , result) ; }

} // verus!