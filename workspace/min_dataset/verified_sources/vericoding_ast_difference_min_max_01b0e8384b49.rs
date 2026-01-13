use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn difference_min_max_precond (a : & Vec < i32 >) -> bool { a . len () > 0 }
fn difference_min_max (a : & Vec < i32 >) -> (result : i32) requires difference_min_max_precond (a) { return 0 ; }

} // verus!