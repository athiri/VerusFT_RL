use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn merge_intervals_precond (intervals : Seq < (int , int) >) -> bool { true }
fn merge_intervals (intervals : Vec < (int , int) >) -> (result : Vec < (int , int) >) requires merge_intervals_precond (intervals @) { return Vec :: new () ; }

} // verus!