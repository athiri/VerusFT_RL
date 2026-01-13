use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn is_prime_precond (n : nat) -> bool { n >= 2 }
fn is_prime (n : u32) -> (result : bool) requires is_prime_precond (n as nat) , n <= 0xFFFF , { return false ; }

} // verus!