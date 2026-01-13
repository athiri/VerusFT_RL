use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn max_element (a : & Vec < i32 >) -> (max : i32) requires a . len () > 0 , ensures forall | i : int | 0 <= i < a . len () ==> a [i] <= max , exists | i : int | 0 <= i < a . len () && a [i] == max , { let mut max = a [0] ; let mut idx = 1 ; while idx < a . len () invariant 1 <= idx <= a . len () , forall | i : int | 0 <= i < idx ==> a [i] <= max , exists | i : int | 0 <= i < idx && a [i] == max , decreases a . len () - idx { if a [idx] > max { max = a [idx] ; } idx += 1 ; } max }

} // verus!