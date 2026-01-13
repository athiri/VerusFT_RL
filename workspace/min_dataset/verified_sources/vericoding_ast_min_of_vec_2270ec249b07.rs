use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn min_of_vec (v : & Vec < i32 >) -> (min : i32) requires v . len () > 0 , ensures exists | i : int | 0 <= i < v . len () && v [i] == min , forall | i : int | 0 <= i < v . len () ==> min <= v [i] , { let mut min_val = v [0] ; let mut idx = 1 ; while idx < v . len () invariant 1 <= idx <= v . len () , exists | i : int | 0 <= i < idx && v [i] == min_val , forall | i : int | 0 <= i < idx ==> min_val <= v [i] , decreases v . len () - idx { if v [idx] < min_val { min_val = v [idx] ; } idx += 1 ; } min_val }

} // verus!