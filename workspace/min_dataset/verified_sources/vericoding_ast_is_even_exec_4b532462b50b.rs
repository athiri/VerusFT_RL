use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn is_even (n : int) -> bool { n % 2 == 0 }
fn is_even_exec (n : i32) -> (result : bool) ensures result == is_even (n as int) { n % 2 == 0 }

} // verus!