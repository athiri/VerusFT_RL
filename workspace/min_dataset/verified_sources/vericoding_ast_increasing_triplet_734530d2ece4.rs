use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn increasing_triplet_precond (nums : Seq < i32 >) -> bool { true }
fn increasing_triplet (nums : Vec < i32 >) -> (result : bool) requires increasing_triplet_precond (nums @) { return false ; }

} // verus!