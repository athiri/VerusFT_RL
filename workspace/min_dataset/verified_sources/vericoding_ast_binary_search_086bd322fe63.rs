use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn binary_search (a : & Vec < i32 > , circle : i32) -> (n : usize) requires true , ensures 0 <= n <= a . len () , { let mut low : usize = 0 ; let mut high : usize = a . len () ; while low < high invariant 0 <= low <= high <= a . len () , decreases high - low , { let mid = low + (high - low) / 2 ; if a [mid] == circle { return mid ; } else if a [mid] < circle { low = mid + 1 ; } else { high = mid ; } } low }

} // verus!