use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn count_sum_divisible_by_spec (n : nat , d : nat) -> nat recommends d > 0 decreases n { if n == 0 { 0nat } else { let prev = (n - 1) as nat ; count_sum_divisible_by_spec (prev , d) + (if is_sum_divisible_by (prev , d) { 1nat } else { 0nat }) } }
spec fn count_sum_divisible_by_precond (n : nat , d : nat) -> bool { d > 0 }
spec fn is_sum_divisible_by (x : nat , d : nat) -> bool recommends d > 0 { sum_of_digits (x) % d == 0 }
spec fn sum_of_digits (x : nat) -> nat decreases x { if x == 0 { 0nat } else { (x % 10) + sum_of_digits (x / 10) } }
fn is_sum_divisible_by_exec (x : u32 , d : u32) -> (result : bool) requires d > 0 ensures result == is_sum_divisible_by (x as nat , d as nat) { let sum = sum_of_digits_exec (x) ; sum % d == 0 }
# [verifier :: external_body] fn sum_of_digits_exec (x : u32) -> (result : u32) ensures result == sum_of_digits (x as nat) { let mut n = x ; let mut sum = 0u32 ; while n > 0 { sum = sum + (n % 10) ; n = n / 10 ; } sum }
# [verifier :: external_body] fn count_sum_divisible_by (n : u32 , d : u32) -> (result : u32) requires count_sum_divisible_by_precond (n as nat , d as nat) ensures result == count_sum_divisible_by_spec (n as nat , d as nat) { let mut count = 0u32 ; let mut i = 0u32 ; while i < n { if is_sum_divisible_by_exec (i , d) { count = count + 1 ; } i = i + 1 ; } count }

} // verus!