use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn smallest_list_length (list : & Vec < Vec < i32 > >) -> (min : usize) requires list . len () > 0 , ensures min >= 0 , forall | i : int | 0 <= i < list . len () ==> min <= # [trigger] list [i] . len () , exists | i : int | 0 <= i < list . len () && min == # [trigger] list [i] . len () , { let mut min = list [0] . len () ; let mut j = 1 ; while j < list . len () invariant 1 <= j <= list . len () , min >= 0 , forall | i : int | 0 <= i < j ==> min <= # [trigger] list [i] . len () , exists | i : int | 0 <= i < j && min == # [trigger] list [i] . len () , decreases list . len () - j , { if list [j] . len () < min { min = list [j] . len () ; } j += 1 ; } min }

} // verus!