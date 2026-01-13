use vstd::prelude::*;
use vstd :: arithmetic :: power :: pow ;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
pub open spec fn if_power_of_four_precond (n : nat) -> bool { true }
pub fn if_power_of_four (n : u32) -> (result : bool) requires if_power_of_four_precond (n as nat) { return false ; }

} // verus!