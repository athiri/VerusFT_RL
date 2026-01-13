use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn main_method (x_init : u32 , y : u32) -> (z : u32) requires (x_init as int) * (y as int) <= MAX as int , ensures z == 0 { return 0 ; }

} // verus!