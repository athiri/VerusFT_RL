use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn two_sum_precond (nums : Seq < i32 > , target : i32) -> bool { true }
fn two_sum (nums : Vec < i32 > , target : i32) -> (result : Option < (usize , usize) >) requires two_sum_precond (nums @ , target) { return None ; }

} // verus!