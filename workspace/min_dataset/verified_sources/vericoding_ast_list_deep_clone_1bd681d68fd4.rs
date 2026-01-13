use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn list_deep_clone (arr : & Vec < u64 >) -> (copied : Vec < u64 >) ensures arr @ . len () == copied @ . len () , forall | i : int | (0 <= i < arr . len ()) ==> arr [i] == copied [i] , { let mut copied = Vec :: new () ; let mut i = 0 ; while i < arr . len () invariant copied @ . len () == i , forall | j : int | (0 <= j < i) ==> arr [j] == copied [j] , i <= arr @ . len () , decreases arr @ . len () - i { copied . push (arr [i]) ; i += 1 ; } assert (i == arr @ . len ()) ; assert (copied @ . len () == i) ; copied }

} // verus!