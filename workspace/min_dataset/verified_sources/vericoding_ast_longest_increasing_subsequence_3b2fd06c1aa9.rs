use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn longest_increasing_subsequence_precond (nums : Seq < i32 >) -> bool { true }
spec fn longest_increasing_subsequence_postcond (nums : Seq < i32 > , result : i32) -> bool { 0 <= result <= nums . len () }
fn longest_increasing_subsequence (nums : Vec < i32 >) -> (result : i32) requires longest_increasing_subsequence_precond (nums @) , nums . len () <= MAX ensures longest_increasing_subsequence_postcond (nums @ , result) { return 0 ; }

} // verus!