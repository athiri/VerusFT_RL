use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn positive (s : Seq < i32 >) -> bool { forall | u : int | 0 <= u < s . len () ==> s [u] >= 0 }
fn mpositivertl (v : & [i32]) -> (b : bool) ensures b == positive (v @) { let mut i = v . len () ; while i > 0 invariant 0 <= i <= v . len () , forall | u : int | i <= u < v . len () ==> v [u] >= 0 decreases i { i = i - 1 ; assert (0 <= i < v . len ()) ; if v [i] < 0 { return false ; } } assert (i == 0) ; assert (forall | u : int | 0 <= u < v . len () ==> v [u] >= 0) ; true }

} // verus!