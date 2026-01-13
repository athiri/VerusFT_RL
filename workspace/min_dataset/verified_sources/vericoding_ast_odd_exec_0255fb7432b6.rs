use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn odd (n : int) -> bool { n % 2 == 1 }
fn odd_exec (n : u32) -> (result : bool) ensures result == odd (n as int) { n % 2 == 1 }

} // verus!