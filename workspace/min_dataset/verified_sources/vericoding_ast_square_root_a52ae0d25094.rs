use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn square_root_precond (n : nat) -> bool { true }
fn square_root (n : u32) -> (result : u32) requires square_root_precond (n as nat) , n <= 100 , { return 0 ; }

} // verus!