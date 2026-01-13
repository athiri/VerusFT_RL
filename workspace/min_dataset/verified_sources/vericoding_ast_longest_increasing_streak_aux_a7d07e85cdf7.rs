use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn longest_increasing_streak_aux (nums : & Vec < i32 > , idx : usize , prev : Option < i32 > , curr_len : usize , max_len : usize) -> (result : usize) requires idx <= nums . len () , curr_len <= nums . len () , max_len <= nums . len () ensures result <= nums . len () decreases nums . len () - idx { return 0 ; }

} // verus!