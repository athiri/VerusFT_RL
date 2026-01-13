use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn bounded_loop (bound : u32 , r : u32 , n : u32) -> (result : u32) requires bound <= 100 , r <= 10 , n <= 100 , decreases bound { return 0 ; }

} // verus!