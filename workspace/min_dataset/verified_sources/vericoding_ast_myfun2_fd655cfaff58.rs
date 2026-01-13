use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
pub fn myfun2 (x : & mut Vec < i32 >) requires forall | k : int | 0 <= k < old (x) . len () ==> old (x) [k] <= 0x7FFF_FFFB , ensures x @ . len () == old (x) @ . len () , forall | k : int | 0 <= k < x . len () ==> # [trigger] x @ [k] == old (x) @ [k] + 4 , { let mut i = 0 ; let ghost original_x = x @ ; while i < x . len () invariant 0 <= i <= x . len () , x @ . len () == old (x) @ . len () , original_x == old (x) @ , forall | k : int | 0 <= k < i ==> # [trigger] x @ [k] == original_x [k] + 4 , forall | k : int | i <= k < x . len () ==> # [trigger] x @ [k] == original_x [k] , forall | k : int | 0 <= k < x . len () ==> original_x [k] <= 0x7FFF_FFFB , decreases x . len () - i { let old_val = x [i] ; assert (original_x [i as int] == old_val) ; assert (old_val <= 0x7FFF_FFFB) ; x . set (i , old_val + 4) ; assert (forall | k : int | 0 <= k < (i + 1) ==> # [trigger] x @ [k] == original_x [k] + 4) ; assert (forall | k : int | (i + 1) <= k < x . len () ==> # [trigger] x @ [k] == original_x [k]) ; i += 1 ; } assert (forall | k : int | 0 <= k < x . len () ==> # [trigger] x @ [k] == old (x) @ [k] + 4) ; }

} // verus!