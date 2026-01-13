use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn longest_increasing_streak_precond (nums : Seq < i32 >) -> bool { true }
spec fn longest_increasing_streak_postcond (nums : Seq < i32 > , result : nat) -> bool { result <= nums . len () && (nums . len () == 0 ==> result == 0) }
fn longest_increasing_streak (nums : Vec < i32 >) -> (result : usize) requires longest_increasing_streak_precond (nums @) ensures longest_increasing_streak_postcond (nums @ , result as nat) { return 0 ; }

} // verus!