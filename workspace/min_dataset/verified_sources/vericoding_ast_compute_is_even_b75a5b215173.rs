use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn compute_is_even (x : u32) -> (is_even : bool) ensures (x % 2 == 0) == is_even { x % 2 == 0 }

} // verus!