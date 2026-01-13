use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn sum_of_fourth_power_of_odd_numbers_precond (n : nat) -> bool { true }
spec fn sum_of_fourth_power_of_odd_numbers_spec (n : nat) -> nat decreases n { if n == 0 { 0nat } else { let prev = sum_of_fourth_power_of_odd_numbers_spec ((n - 1) as nat) ; let next_odd = (2 * (n - 1) + 1) as nat ; let next_odd_fourth = (next_odd * next_odd * next_odd * next_odd) as nat ; (prev + next_odd_fourth) as nat } }
fn sum_of_fourth_power_of_odd_numbers (n : u32) -> (result : u32) requires sum_of_fourth_power_of_odd_numbers_precond (n as nat) , n <= 1 ensures result as nat == sum_of_fourth_power_of_odd_numbers_spec (n as nat) decreases n { if n == 0 { 0 } else { let prev = sum_of_fourth_power_of_odd_numbers (n - 1) ; let next_odd = 2 * (n - 1) + 1 ; let next_odd_fourth = next_odd * next_odd * next_odd * next_odd ; prev + next_odd_fourth } }

} // verus!