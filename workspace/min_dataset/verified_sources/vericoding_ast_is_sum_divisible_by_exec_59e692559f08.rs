use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn is_sum_divisible_by (x : nat , d : nat) -> bool recommends d > 0 { sum_of_digits (x) % d == 0 }
spec fn sum_of_digits (x : nat) -> nat decreases x { if x == 0 { 0nat } else { (x % 10) + sum_of_digits (x / 10) } }
# [verifier :: external_body] fn sum_of_digits_exec (x : u32) -> (result : u32) ensures result == sum_of_digits (x as nat) { let mut n = x ; let mut sum = 0u32 ; while n > 0 { sum = sum + (n % 10) ; n = n / 10 ; } sum }
fn is_sum_divisible_by_exec (x : u32 , d : u32) -> (result : bool) requires d > 0 ensures result == is_sum_divisible_by (x as nat , d as nat) { let sum = sum_of_digits_exec (x) ; sum % d == 0 }

} // verus!