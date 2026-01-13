use vstd::prelude::*;
# [allow (unused_imports)] use vstd :: prelude :: * ;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn linear_search (nums : Vec < i32 > , target : i32) -> (ret : i32) requires nums @ . len () < 0x8000_0000 , ensures ret < nums @ . len () , ret >= 0 ==> nums @ [ret as int] == target , ret >= 0 ==> forall | i : int | 0 <= i < ret as int ==> # [trigger] nums @ [i] != target , ret < 0 ==> forall | i : int | 0 <= i < nums @ . len () as int ==> # [trigger] nums @ [i] != target , { let mut i = 0 ; while i < nums . len () invariant 0 <= i <= nums @ . len () , forall | j : int | 0 <= j < i ==> # [trigger] nums @ [j] != target , i <= nums @ . len () && nums @ . len () < 0x8000_0000 , decreases nums @ . len () - i { if nums [i] == target { assert (nums @ [i as int] == target) ; assert (i < nums @ . len ()) ; assert (i < 0x8000_0000) ; return i as i32 ; } i += 1 ; } assert (forall | j : int | 0 <= j < nums @ . len () ==> nums @ [j] != target) ; return - 1 ; }

} // verus!