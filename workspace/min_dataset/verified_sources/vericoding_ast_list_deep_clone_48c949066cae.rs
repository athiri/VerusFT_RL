use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn list_deep_clone (arr : & Vec < u64 >) -> (copied : Vec < u64 >) ensures arr @ . len () == copied @ . len () , forall | i : int | (0 <= i < arr . len ()) ==> arr [i] == copied [i] , { let mut copied = Vec :: new () ; let mut index = 0 ; while index < arr . len () invariant index <= arr . len () , copied @ . len () == index , forall | i : int | (0 <= i < index) ==> arr [i] == copied [i] , decreases arr . len () - index { copied . push (arr [index]) ; index += 1 ; } copied }

} // verus!