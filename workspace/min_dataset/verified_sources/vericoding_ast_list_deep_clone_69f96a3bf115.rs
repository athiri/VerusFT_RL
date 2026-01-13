use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn list_deep_clone (arr : & Vec < u64 >) -> (copied : Vec < u64 >) ensures arr @ . len () == copied @ . len () , forall | i : int | (0 <= i < arr . len ()) ==> arr [i] == copied [i] , { let mut copied = Vec :: new () ; let mut idx = 0 ; while idx < arr . len () invariant idx <= arr . len () , copied @ . len () == idx , forall | i : int | (0 <= i < idx) ==> arr [i] == copied [i] , decreases arr . len () - idx { copied . push (arr [idx]) ; idx += 1 ; } copied }

} // verus!