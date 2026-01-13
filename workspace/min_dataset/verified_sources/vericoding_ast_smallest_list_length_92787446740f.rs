use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
# [verifier :: loop_isolation (false)] fn smallest_list_length (lists : Vec < Vec < i32 > >) -> (result : usize) requires lists . len () > 0 , ensures exists | i : int | # ! [auto] 0 <= i < lists . len () && result == lists [i] . len () , forall | i : int | # ! [auto] 0 <= i < lists . len () ==> result <= lists [i] . len () , { let mut min_length = lists [0] . len () ; let mut index = 1 ; while index < lists . len () invariant lists . len () > 0 , 1 <= index <= lists . len () , exists | j : int | # ! [auto] 0 <= j < index && min_length == lists [j] . len () , forall | j : int | # ! [auto] 0 <= j < index ==> min_length <= lists [j] . len () , decreases lists . len () - index { if lists [index] . len () < min_length { min_length = lists [index] . len () ; } index += 1 ; } min_length }

} // verus!