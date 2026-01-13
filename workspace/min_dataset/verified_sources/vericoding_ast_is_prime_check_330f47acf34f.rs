use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn is_prime_check (n : u32 , i : u32 , fuel : u32) -> (result : bool) requires n >= 2 , n <= 0xFFFF , i >= 2 , fuel <= n , decreases fuel , { return false ; }

} // verus!