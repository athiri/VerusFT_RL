use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn copy (arr : & Vec < i32 >) -> (ret : Vec < i32 >) ensures ret . len () == arr . len () , forall | i : int | 0 <= i < arr . len () ==> ret [i] == arr [i] , { let mut result = Vec :: new () ; let mut i = 0 ; while i < arr . len () invariant i <= arr . len () , result . len () == i , forall | j : int | 0 <= j < i ==> result [j] == arr [j] , decreases arr . len () - i { result . push (arr [i]) ; i += 1 ; } result }

} // verus!