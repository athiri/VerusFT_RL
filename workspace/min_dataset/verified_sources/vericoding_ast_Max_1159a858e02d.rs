use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn Max (x : u32 , y : u32) -> (r : u32) ensures r >= x && r >= y , r == x || r == y , { if x >= y { x } else { y } }

} // verus!