use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
pub fn myfun (a : & mut Vec < i32 > , sum : & mut Vec < i32 > , N : usize) requires N > 0 , old (a) . len () == N , old (sum) . len () == 1 , N < 1000 , ensures sum [0] == 2 * N , { sum . set (0 , 2 * N as i32) ; }

} // verus!