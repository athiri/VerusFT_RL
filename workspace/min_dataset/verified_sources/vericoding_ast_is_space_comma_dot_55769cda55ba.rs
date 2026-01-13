use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn is_space_comma_dot_spec (c : char) -> bool { c == ' ' || c == ',' || c == '.' }
fn is_space_comma_dot (c : char) -> (result : bool) ensures result == is_space_comma_dot_spec (c) { c == ' ' || c == ',' || c == '.' }

} // verus!