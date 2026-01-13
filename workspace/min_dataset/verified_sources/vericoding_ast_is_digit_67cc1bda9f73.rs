use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn is_digit_sepc (c : u8) -> bool { c >= 48 && c <= 57 }
fn is_digit (c : u8) -> (res : bool) ensures res == is_digit_sepc (c) , { c >= 48 && c <= 57 }

} // verus!