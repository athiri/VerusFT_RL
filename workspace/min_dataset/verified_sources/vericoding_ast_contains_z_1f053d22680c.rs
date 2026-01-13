use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn contains_z (text : & [u8]) -> (result : bool) ensures result == (exists | i : int | 0 <= i < text . len () && (text [i] == 90 || text [i] == 122)) , { let mut i = 0 ; while i < text . len () invariant 0 <= i <= text . len () , forall | j : int | 0 <= j < i ==> ! (text [j] == 90 || text [j] == 122) , decreases text . len () - i , { if text [i] == 90 || text [i] == 122 { return true ; } i += 1 ; } false }

} // verus!