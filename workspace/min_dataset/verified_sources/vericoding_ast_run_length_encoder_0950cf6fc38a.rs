use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn run_length_encoder_precond (input : Seq < char >) -> bool { true }
fn run_length_encoder (input : Vec < char >) -> (result : Vec < char >) requires run_length_encoder_precond (input @) { return Vec :: new () ; }

} // verus!