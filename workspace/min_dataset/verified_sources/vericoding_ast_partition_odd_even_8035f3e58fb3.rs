use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn odd (n : int) -> bool { n % 2 == 1 }
fn odd_exec (n : u32) -> (result : bool) ensures result == odd (n as int) { n % 2 == 1 }
fn partition_odd_even (a : & mut Vec < u32 >) requires old (a) . len () > 0 , ensures a @ . len () == old (a) @ . len () , { let mut left = 0 ; let mut right = a . len () ; while left < right invariant left <= right , right <= a @ . len () , a @ . len () == old (a) @ . len () , decreases right - left { if odd_exec (a [left]) { left += 1 ; } else { right -= 1 ; let temp = a [left] ; let right_val = a [right] ; a . set (left , right_val) ; a . set (right , temp) ; } } }

} // verus!