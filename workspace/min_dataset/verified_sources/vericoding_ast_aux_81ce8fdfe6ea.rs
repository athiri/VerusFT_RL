use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn aux (m : i32 , fuel : u32) -> (result : bool) requires m > 0 , decreases fuel { if fuel == 0 { false } else if m == 1 { true } else if m % 2 == 1 { false } else { aux (m / 2 , fuel - 1) } }

} // verus!