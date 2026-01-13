use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn contains_z (text : & [u8]) -> (result : bool) ensures result == (exists | i : int | 0 <= i < text . len () && (text [i] == 90 || text [i] == 122)) , { for i in 0 .. text . len () invariant forall | j : int | 0 <= j < i ==> ! (text [j] == 90 || text [j] == 122) , { if text [i] == 90 || text [i] == 122 { return true ; } } false }

} // verus!