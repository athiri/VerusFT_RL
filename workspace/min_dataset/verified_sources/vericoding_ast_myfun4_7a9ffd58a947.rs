use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
pub fn myfun4 (x : & Vec < u64 > , y : & mut Vec < u64 >) requires old (y) . len () == 0 , ensures forall | k : int | 0 <= k < y . len () ==> y [k] % 3 == 0 && x @ . contains (y @ [k]) , { let mut i = 0 ; while i < x . len () invariant 0 <= i <= x . len () , forall | k : int | 0 <= k < y . len () ==> y [k] % 3 == 0 && x @ . contains (y @ [k]) , decreases x . len () - i { if x [i] % 3 == 0 { y . push (x [i]) ; } i += 1 ; } }

} // verus!