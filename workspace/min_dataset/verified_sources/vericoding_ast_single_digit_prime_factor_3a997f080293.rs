use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn single_digit_prime_factor_precond (n : nat) -> bool { true }
spec fn single_digit_prime_factor_postcond (n : nat , result : nat) -> bool { (result == 0 || result == 2 || result == 3 || result == 5 || result == 7) && (result == 0 ==> (n == 0 || (n % 2 != 0 && n % 3 != 0 && n % 5 != 0 && n % 7 != 0))) && (result != 0 ==> (n != 0 && n % result == 0 && smaller_prime_factors_dont_divide (n , result))) }
spec fn smaller_prime_factors_dont_divide (n : nat , result : nat) -> bool { if result == 2 { true } else if result == 3 { n % 2 != 0 } else if result == 5 { n % 2 != 0 && n % 3 != 0 } else if result == 7 { n % 2 != 0 && n % 3 != 0 && n % 5 != 0 } else { true } }
fn single_digit_prime_factor (n : u32) -> (result : u32) requires single_digit_prime_factor_precond (n as nat) , ensures single_digit_prime_factor_postcond (n as nat , result as nat) , { if n == 0 { return 0 ; } if n % 2 == 0 { return 2 ; } if n % 3 == 0 { return 3 ; } if n % 5 == 0 { return 5 ; } if n % 7 == 0 { return 7 ; } return 0 ; }

} // verus!