use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
# [verifier :: loop_isolation (false)] fn helper (lst : & Vec < i32 > , cur_max : i32 , global_max : i32 , index : usize) -> (result : i32) requires index <= lst . len () , lst . len () <= 100 decreases lst . len () - index { return 0 ; }

} // verus!