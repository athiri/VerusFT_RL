use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn longest_increasing_subsequence_postcond (numbers : Seq < int > , result : usize) -> bool { true }
spec fn longest_increasing_subsequence_precond (numbers : Seq < int >) -> bool { true }
fn longest_increasing_subsequence (numbers : Vec < i32 >) -> (result : usize) requires longest_increasing_subsequence_precond (numbers @ . map (| i : int , x : i32 | x as int)) , numbers . len () < 1000000 , ensures longest_increasing_subsequence_postcond (numbers @ . map (| i : int , x : i32 | x as int) , result) { return 0 ; }

} // verus!