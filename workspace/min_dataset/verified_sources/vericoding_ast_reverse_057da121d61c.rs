use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn reverse (a : & [i32]) -> (result : Vec < i32 >) ensures result . len () == a . len () , forall | i : int | 0 <= i && i < result . len () ==> result [i] == a [a . len () - 1 - i] , { let mut result = Vec :: new () ; let mut j = a . len () ; while j > 0 invariant result . len () == a . len () - j , forall | i : int | 0 <= i && i < result . len () ==> result [i] == a [a . len () - 1 - i] , decreases j { j = j - 1 ; result . push (a [j]) ; } result }

} // verus!