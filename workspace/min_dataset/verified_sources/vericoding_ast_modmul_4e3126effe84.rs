use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn modmul (a : u32 , b : u32 , p : u32) -> (mul : u32) by (nonlinear_arith) requires p > 0 , ensures mul == ((a as int) * (b as int)) % (p as int) , { let result = ((a as u64) * (b as u64)) % (p as u64) ; result as u32 }

} // verus!