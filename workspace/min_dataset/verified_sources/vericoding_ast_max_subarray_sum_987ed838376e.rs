use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn max_subarray_sum_precond (xs : Seq < i32 >) -> bool { true }
fn max_subarray_sum (xs : Vec < i32 >) -> (result : i32) requires max_subarray_sum_precond (xs @) , xs . len () <= 100 { return 0 ; }

} // verus!