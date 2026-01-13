use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn last_digit_precond (n : nat) -> bool { true }
spec fn last_digit_spec (n : nat) -> nat { n % 10 }
fn last_digit (n : u32) -> (result : u32) requires last_digit_precond (n as nat) , ensures 0 <= result < 10 , result == last_digit_spec (n as nat) , { n % 10 }

} // verus!