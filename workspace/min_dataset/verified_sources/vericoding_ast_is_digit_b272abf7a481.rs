use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn is_digit_sepc (c : char) -> (res : bool) { (c as u32) >= 48 && (c as u32) <= 57 }
fn is_digit (c : char) -> (res : bool) ensures res == is_digit_sepc (c) , { (c as u32) >= 48 && (c as u32) <= 57 }

} // verus!