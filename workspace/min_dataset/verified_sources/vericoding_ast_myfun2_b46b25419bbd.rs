use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
pub fn myfun2 (x : & mut Vec < i32 >) requires forall | k : int | 0 <= k < old (x) . len () ==> old (x) [k] <= 0x7FFF_FFFB , ensures x @ . len () == old (x) @ . len () , forall | k : int | 0 <= k < x . len () ==> # [trigger] x @ [k] == old (x) @ [k] + 4 , { let mut i = 0 ; while i < x . len () invariant 0 <= i <= x . len () , x @ . len () == old (x) @ . len () , forall | k : int | 0 <= k < i ==> # [trigger] x @ [k] == old (x) @ [k] + 4 , forall | k : int | i <= k < x . len () ==> # [trigger] x @ [k] == old (x) @ [k] , forall | k : int | i <= k < x . len () ==> x @ [k] <= 0x7FFF_FFFB , decreases x . len () - i { let old_val = x [i] ; assert (old_val <= 0x7FFF_FFFB) ; assert (old_val + 4 <= 0x7FFF_FFFF) ; x . set (i , old_val + 4) ; assert (forall | k : int | 0 <= k < i + 1 ==> # [trigger] x @ [k] == old (x) @ [k] + 4) ; i += 1 ; } }

} // verus!