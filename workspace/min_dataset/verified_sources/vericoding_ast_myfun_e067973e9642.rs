use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn myfun (a : & mut Vec < i32 > , sum : & mut Vec < i32 > , N : i32) requires N > 0 , old (a) . len () == N , old (sum) . len () == 1 , N < 1000 , ensures sum [0] == 6 * N , { sum . set (0 , 6 * N) ; }

} // verus!