use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn contains (str : & Vec < i32 > , key : i32) -> (result : bool) ensures result <==> (exists | i : int | 0 <= i < str . len () && (str [i] == key)) , { let mut idx = 0 ; while idx < str . len () invariant 0 <= idx <= str . len () , forall | j : int | 0 <= j < idx ==> str [j] != key , decreases str . len () - idx , { if str [idx] == key { return true ; } idx += 1 ; } false }

} // verus!