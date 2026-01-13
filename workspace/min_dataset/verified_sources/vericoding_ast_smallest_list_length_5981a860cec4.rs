use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
# [verifier :: loop_isolation (false)] fn smallest_list_length (lists : Vec < Vec < i32 > >) -> (result : usize) requires lists . len () > 0 , ensures exists | i : int | # ! [auto] 0 <= i < lists . len () && result == lists [i] . len () , forall | i : int | # ! [auto] 0 <= i < lists . len () ==> result <= lists [i] . len () , { let mut min_length = lists [0] . len () ; let mut i = 1 ; while i < lists . len () invariant 1 <= i <= lists . len () , exists | j : int | # ! [auto] 0 <= j < i && min_length == lists [j] . len () , forall | j : int | # ! [auto] 0 <= j < i ==> min_length <= lists [j] . len () , decreases lists . len () - i { if lists [i] . len () < min_length { min_length = lists [i] . len () ; } i += 1 ; } min_length }

} // verus!