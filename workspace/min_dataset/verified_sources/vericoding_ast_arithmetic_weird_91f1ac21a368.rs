use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
# [verifier :: loop_isolation (false)] fn arithmetic_weird () -> (result : i32) ensures result < 10 { 0 }

} // verus!