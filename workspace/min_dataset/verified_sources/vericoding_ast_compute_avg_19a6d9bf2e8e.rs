use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn compute_avg (a : u32 , b : u32) -> (avg : u32) requires a < 0x80000000 && b < 0x80000000 ensures avg == (a + b) / 2 { (a + b) / 2 }

} // verus!