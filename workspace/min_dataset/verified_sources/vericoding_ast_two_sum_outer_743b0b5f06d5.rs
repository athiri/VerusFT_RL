use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn two_sum_outer (nums : & Vec < i32 > , target : i32 , i : usize) -> Option < (usize , usize) > decreases nums . len () - i { return None ; }

} // verus!