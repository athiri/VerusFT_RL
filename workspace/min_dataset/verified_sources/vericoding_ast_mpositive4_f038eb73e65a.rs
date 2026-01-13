use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn positive (s : Seq < i32 >) -> bool { forall | u : int | 0 <= u < s . len () ==> s [u] >= 0 }
fn mpositive4 (v : & [i32]) -> (b : bool) ensures b == positive (v @) { let mut result = true ; for i in 0 .. v . len () invariant result == (forall | u : int | 0 <= u < i ==> v [u] >= 0) { if v [i] < 0 { result = false ; } } result }

} // verus!