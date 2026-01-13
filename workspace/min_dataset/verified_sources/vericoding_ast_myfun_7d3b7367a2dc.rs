use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
pub fn myfun (a : & mut Vec < i32 > , sum : & mut Vec < i32 > , N : i32) requires N > 0 , N <= MAX / 2 , old (a) . len () == N , old (sum) . len () == 1 , ensures sum [0] == 2 * N , { assert (2 * N <= MAX) ; sum . set (0 , 2 * N) ; }

} // verus!