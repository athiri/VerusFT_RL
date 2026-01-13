use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn sum_powers_helper (n : u32 , k : u32 , acc : u32) -> u32 decreases n , { return 0 ; }

} // verus!