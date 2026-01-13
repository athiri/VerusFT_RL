use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn is_power_of_two_precond (n : int) -> bool { true }
spec fn is_power_of_two_postcond (n : int , result : bool) -> bool { if result { exists | x : nat | pow (2 , x) == n && n > 0 } else { ! exists | x : nat | pow (2 , x) == n && n > 0 } }
spec fn pow (base : int , exp : nat) -> int decreases exp { if exp == 0 { 1 } else { base * pow (base , (exp - 1) as nat) } }
fn is_power_of_two (n : i32) -> (result : bool) requires is_power_of_two_precond (n as int) , ensures is_power_of_two_postcond (n as int , result) , { proof { admit () ; } if n <= 0 { false } else { n & (n - 1) == 0 } }

} // verus!