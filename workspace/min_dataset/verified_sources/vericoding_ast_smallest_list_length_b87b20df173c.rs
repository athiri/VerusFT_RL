use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn smallest_list_length (list : & Vec < Vec < i32 > >) -> (min : usize) requires list . len () > 0 , ensures min >= 0 , forall | i : int | 0 <= i < list . len () ==> min <= # [trigger] list [i] . len () , exists | i : int | 0 <= i < list . len () && min == # [trigger] list [i] . len () , { let mut min = list [0] . len () ; let mut idx = 1 ; while idx < list . len () invariant 0 <= idx <= list . len () , forall | i : int | 0 <= i < idx ==> min <= # [trigger] list [i] . len () , exists | i : int | 0 <= i < idx && min == # [trigger] list [i] . len () , decreases list . len () - idx { if list [idx] . len () < min { min = list [idx] . len () ; } idx += 1 ; } min }

} // verus!