use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn contains_z (text : & Vec < char >) -> (result : bool) ensures result == (exists | i : int | 0 <= i < text . len () && (text [i] == 'Z' || text [i] == 'z')) , { for i in 0 .. text . len () invariant ! (exists | j : int | 0 <= j < i && (text [j] == 'Z' || text [j] == 'z')) , { if text [i] == 'Z' || text [i] == 'z' { return true ; } } false }

} // verus!