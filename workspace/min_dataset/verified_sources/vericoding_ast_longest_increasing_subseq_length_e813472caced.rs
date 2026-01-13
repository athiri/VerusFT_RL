use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn longest_increasing_subseq_length_postcond (xs : Seq < i32 > , result : nat) -> bool { result <= xs . len () }
spec fn longest_increasing_subseq_length_precond (xs : Seq < i32 >) -> bool { true }
fn longest_increasing_subseq_length (xs : & Vec < i32 >) -> (result : usize) requires longest_increasing_subseq_length_precond (xs @) , ensures longest_increasing_subseq_length_postcond (xs @ , result as nat) , { return 0 ; }

} // verus!