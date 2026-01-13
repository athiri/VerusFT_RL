use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
pub fn myfun (a : & mut Vec < usize > , sum : & mut Vec < usize > , N : usize) requires old (a) . len () == N , old (sum) . len () == 1 , N > 0 , ensures sum [0] == 0 , { sum . set (0 , 0) ; }

} // verus!