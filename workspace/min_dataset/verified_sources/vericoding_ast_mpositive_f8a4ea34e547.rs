use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn positive (s : Seq < i32 >) -> bool { forall | u : int | 0 <= u < s . len () ==> s [u] >= 0 }
fn mpositive (v : & [i32]) -> (b : bool) ensures b == positive (v @) { for i in 0 .. v . len () invariant forall | u : int | 0 <= u < i ==> v [u] >= 0 { if v [i] < 0 { return false ; } } true }

} // verus!