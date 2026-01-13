use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn update_elements (a : & mut Vec < i32 >) requires old (a) . len () >= 8 , old (a) [4] + 3 <= MAX , ensures old (a) [4] + 3 == a [4] , a [7] == 516 , forall | i : int | 0 <= i < a . len () && i != 7 && i != 4 ==> a [i] == old (a) [i] , { a [4] = a [4] + 3 ; a [7] = 516 ; }

} // verus!