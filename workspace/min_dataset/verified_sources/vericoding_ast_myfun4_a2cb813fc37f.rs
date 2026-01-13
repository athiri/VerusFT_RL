use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
pub fn myfun4 (x : & Vec < u64 > , y : & mut Vec < u64 >) requires old (y) . len () == 0 , ensures forall | k : int | 0 <= k < y . len () ==> y [k] % 3 == 0 && x @ . contains (y @ [k]) , { }

} // verus!