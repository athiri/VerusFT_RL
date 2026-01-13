use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn even (n : int) -> bool { n % 2 == 0 }
fn even_exec (n : u32) -> (result : bool) ensures result == even (n as int) { n % 2 == 0 }

} // verus!