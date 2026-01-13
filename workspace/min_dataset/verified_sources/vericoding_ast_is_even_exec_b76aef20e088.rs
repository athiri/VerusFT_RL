use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn is_even (n : u32) -> bool { (n % 2) == 0 }
fn is_even_exec (n : u32) -> (result : bool) ensures result == is_even (n) { (n % 2) == 0 }

} // verus!