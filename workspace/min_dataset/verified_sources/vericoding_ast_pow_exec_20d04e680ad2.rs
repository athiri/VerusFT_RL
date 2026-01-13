use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn pow_exec (base : u32 , exp : u32) -> u32 decreases exp , { return 0 ; }

} // verus!