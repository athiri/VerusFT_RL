use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn myfun (a : & mut Vec < u32 > , N : u32) -> (sum : u32) requires old (a) . len () == N , N <= 0x7FFF_FFFF , ensures sum <= 2 * N , { 2 * N }

} // verus!