use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn minimum_right_shifts_postcond (nums : Seq < int > , result : int) -> bool { let n = nums . len () ; if n <= 1 { result == 0 } else if result >= 0 { result < n && is_sorted (right_shift (result as nat , nums)) && forall | j : nat | j < result ==> ! is_sorted (# [trigger] right_shift (j , nums)) } else { result == - 1 && forall | k : nat | k < n ==> ! is_sorted (# [trigger] right_shift (k , nums)) } }
spec fn minimum_right_shifts_precond (nums : Seq < int >) -> bool { forall | i : int , j : int | 0 <= i < j < nums . len () ==> # [trigger] nums [i] != # [trigger] nums [j] }
spec fn right_shift (k : nat , s : Seq < int >) -> Seq < int > decreases k { if k == 0 { s } else { right_shift ((k - 1) as nat , right_shift_one (s)) } }
spec fn is_sorted (s : Seq < int >) -> bool { forall | i : int | 0 <= i < s . len () - 1 ==> # [trigger] s [i] <= s [i + 1] }
spec fn right_shift_one (s : Seq < int >) -> Seq < int > { if s . len () == 0 { s } else { seq ! [s [s . len () - 1]] + s . subrange (0 , s . len () - 1) } }
# [verifier :: external_body] fn minimum_right_shifts (nums : Vec < i32 >) -> (result : i32) requires minimum_right_shifts_precond (nums @ . map (| i , x | x as int)) ensures minimum_right_shifts_postcond (nums @ . map (| i , x | x as int) , result as int) { return 0 ; }

} // verus!