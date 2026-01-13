use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn separate (v : & mut Vec < i32 >) -> (result : usize) requires old (v) . len () > 0 { let mut left = 0 ; let mut right = v . len () - 1 ; while left <= right invariant left <= v . len () , right < v . len () , forall | i : int | 0 <= i < left ==> v [i] < 0 , forall | i : int | (right as int) < i < v . len () ==> v [i] >= 0 , decreases (right + 1) - left { if v [left] < 0 { left += 1 ; } else if v [right] >= 0 { if right == 0 { break ; } right -= 1 ; } else { let temp = v [left] ; let right_val = v [right] ; v . set (left , right_val) ; v . set (right , temp) ; left += 1 ; if right == 0 { break ; } right -= 1 ; } } left }

} // verus!