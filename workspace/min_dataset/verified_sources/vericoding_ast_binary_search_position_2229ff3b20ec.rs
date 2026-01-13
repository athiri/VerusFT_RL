use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn binary_search_position (dp : & Vec < i32 > , x : i32) -> (pos : usize) ensures pos <= dp . len () { let mut left : usize = 0 ; let mut right : usize = dp . len () ; while left < right invariant left <= right <= dp . len () decreases right - left { let mid = left + (right - left) / 2 ; if dp [mid] < x { left = mid + 1 ; } else { right = mid ; } } left }

} // verus!