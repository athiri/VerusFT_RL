use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn square_nums (nums : & Vec < i32 >) -> (squared : Vec < i32 >) requires forall | k : int | 0 <= k < nums . len () ==> (0 <= # [trigger] nums [k] * # [trigger] nums [k] < MAX) , ensures nums . len () == squared . len () , forall | k : int | 0 <= k < nums . len () ==> (# [trigger] squared [k] == nums [k] * nums [k]) , { let mut result = Vec :: new () ; let mut i = 0 ; while i < nums . len () invariant i <= nums . len () , result . len () == i , forall | k : int | 0 <= k < i ==> (# [trigger] result [k] == nums [k] * nums [k]) , forall | k : int | 0 <= k < nums . len () ==> (0 <= # [trigger] nums [k] * # [trigger] nums [k] < MAX) decreases nums . len () - i { assert (0 <= nums [i as int] * nums [i as int] < MAX) ; let squared_val = nums [i] * nums [i] ; result . push (squared_val) ; i += 1 ; assert (forall | k : int | 0 <= k < i ==> result [k] == nums [k] * nums [k]) ; } result }

} // verus!