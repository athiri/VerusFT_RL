use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn solution_precond (nums : Seq < u32 >) -> bool { 1 <= nums . len () && nums . len () <= 100 && forall | i : int | 0 <= i < nums . len () ==> 1 <= nums [i] && nums [i] <= 100 }
# [verifier :: exec_allows_no_decreases_clause] fn solution (nums : Vec < u32 >) -> (result : u32) requires solution_precond (nums @) , { return 0 ; }

} // verus!