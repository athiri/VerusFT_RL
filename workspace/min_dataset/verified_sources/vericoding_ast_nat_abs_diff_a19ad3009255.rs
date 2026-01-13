use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn spec_nat_abs_diff (x : i32 , y : i32) -> nat { let diff = (x as int) - (y as int) ; if diff >= 0 { diff as nat } else { (- diff) as nat } }
fn nat_abs_diff (x : i32 , y : i32) -> (result : usize) ensures result as nat == spec_nat_abs_diff (x , y) { let diff = x as i64 - y as i64 ; if diff >= 0 { diff as usize } else { (- diff) as usize } }

} // verus!