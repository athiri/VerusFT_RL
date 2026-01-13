use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn set_to_seq_precond (s : Seq < int >) -> bool { true }
fn set_to_seq (s : Vec < int >) -> (result : Vec < int >) requires set_to_seq_precond (s @) { return Vec :: new () ; }

} // verus!