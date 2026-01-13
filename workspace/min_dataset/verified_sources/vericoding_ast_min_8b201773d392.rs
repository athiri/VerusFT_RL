use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
# [verifier :: external_body] fn min (a : & [i32]) -> (res : i32) requires a . len () > 0 , ensures exists | i : int | 0 <= i < a . len () && res == a [i] && forall | j : int | 0 <= j < a . len () ==> res <= a [j] , { let mut min_val = a [0] ; for i in 1 .. a . len () { if a [i] < min_val { min_val = a [i] ; } } min_val }

} // verus!