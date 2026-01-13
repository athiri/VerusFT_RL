use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn smallest_missing_precond (l : Seq < usize >) -> bool { is_sorted (l) }
spec fn is_sorted (l : Seq < usize >) -> bool { forall | i : int , j : int | 0 <= i < j < l . len () ==> l [i] < l [j] }
fn smallest_missing (l : & Vec < usize >) -> (result : usize) requires smallest_missing_precond (l @) { return 0 ; }

} // verus!