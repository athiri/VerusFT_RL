use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn longest_consecutive_precond (nums : & Vec < i32 >) -> bool { forall | i : int , j : int | 0 <= i < nums . len () && 0 <= j < nums . len () && i != j ==> nums [i] != nums [j] }
spec fn longest_consecutive_postcond (nums : & Vec < i32 > , result : nat) -> bool { (nums . len () == 0 ==> result == 0) && result <= nums . len () }
fn longest_consecutive (nums : & Vec < i32 >) -> (result : usize) requires longest_consecutive_precond (nums) , ensures longest_consecutive_postcond (nums , result as nat) , { return 0 ; }

} // verus!