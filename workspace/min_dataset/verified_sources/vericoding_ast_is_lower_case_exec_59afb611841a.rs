use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn is_lower_case (c : char) -> (result : bool) { c >= 'a' && c <= 'z' }
fn is_lower_case_exec (c : char) -> (result : bool) ensures result == is_lower_case (c) { c >= 'a' && c <= 'z' }

} // verus!