use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn findMax (a : & [i32]) -> (max : i32) requires a . len () > 0 , ensures exists | k : int | 0 <= k < a . len () && max == a [k] , forall | k : int | 0 <= k < a . len () ==> max >= a [k] { let mut max = a [0] ; let mut i = 1 ; while i < a . len () invariant 0 <= i <= a . len () , exists | k : int | 0 <= k < i && max == a [k] , forall | k : int | 0 <= k < i ==> max >= a [k] decreases a . len () - i { if a [i] > max { max = a [i] ; } i += 1 ; } max }

} // verus!