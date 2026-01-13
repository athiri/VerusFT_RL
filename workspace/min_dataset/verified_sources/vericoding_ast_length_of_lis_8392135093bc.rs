use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn length_of_lis_precond (nums : Vec < i32 >) -> bool { true }
fn length_of_lis (nums : Vec < i32 >) -> (result : usize) requires length_of_lis_precond (nums) , { return 0 ; }

} // verus!