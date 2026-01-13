use vstd::prelude::*;
use vstd :: arithmetic :: logarithm :: log ;
use vstd :: arithmetic :: power :: pow ;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
# [verifier :: external_fn_specification] pub fn ex_checked_pow (x : u32 , exp : u32) -> (ret : Option < u32 >) ensures ret . is_some () <==> ret . unwrap () == pow (x as int , exp as nat) , ret . is_none () <==> pow (x as int , exp as nat) > MAX , { x . checked_pow (exp) }

} // verus!