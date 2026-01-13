use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn MaxA (a : & [i32]) -> (m : i32) requires a . len () > 0 , ensures forall | i : int | 0 <= i < a . len () ==> a [i] <= m , exists | i : int | 0 <= i < a . len () && a [i] == m , { let mut max = a [0] ; let mut index = 1 ; while index < a . len () invariant 0 <= index <= a . len () , forall | i : int | 0 <= i < index ==> a [i] <= max , exists | i : int | 0 <= i < index && a [i] == max , decreases a . len () - index , { if a [index] > max { max = a [index] ; } index += 1 ; } max }

} // verus!