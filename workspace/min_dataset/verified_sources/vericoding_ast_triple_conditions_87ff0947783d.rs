use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn triple_conditions (x : u32) -> (r : u32) requires x % 2 == 0 && x <= 0x55555555u32 , ensures r as int == 3 * (x as int) { 3 * x }

} // verus!