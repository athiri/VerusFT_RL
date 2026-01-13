use vstd::prelude::*;
use vstd :: arithmetic :: logarithm :: log ;
use vstd :: arithmetic :: power :: pow ;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
# [verifier :: external_fn_specification] pub fn ex_ilog (x : u32 , base : u32) -> (ret : u32) requires x > 0 , base > 1 , ensures ret == log (base as int , x as int) , { x . ilog (base) }

} // verus!