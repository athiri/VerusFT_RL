use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn find_majority_element_precond (lst : Seq < i32 >) -> bool { true }
fn find_majority_element (lst : Vec < i32 >) -> (result : i32) requires find_majority_element_precond (lst @) , { return 0 ; }

} // verus!