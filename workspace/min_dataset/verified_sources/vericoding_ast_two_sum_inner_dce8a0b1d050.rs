use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn two_sum_inner (nums : & Vec < i32 > , target : i32 , x : i32 , i : usize , j : usize) -> Option < usize > decreases nums . len () - j { return None ; }

} // verus!