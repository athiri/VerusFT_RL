use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn is_digit_spec (c : char) -> bool { c >= '0' && c <= '9' }
fn is_digit (c : char) -> (result : bool) ensures result == is_digit_spec (c) , { c >= '0' && c <= '9' }

} // verus!