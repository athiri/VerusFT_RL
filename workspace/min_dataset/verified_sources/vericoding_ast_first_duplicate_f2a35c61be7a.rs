use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn first_duplicate_precond (lst : Seq < i32 >) -> bool { true }
fn first_duplicate (lst : Vec < i32 >) -> (result : i32) requires first_duplicate_precond (lst @) { return 0 ; }

} // verus!