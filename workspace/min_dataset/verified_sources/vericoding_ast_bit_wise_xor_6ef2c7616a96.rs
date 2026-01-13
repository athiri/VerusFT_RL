use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn bit_wise_xor (arr1 : & Vec < i32 > , arr2 : & Vec < i32 >) -> (result : Vec < i32 >) requires arr1 . len () == arr2 . len () , ensures result . len () == arr1 . len () , forall | i : int | 0 <= i < result . len () ==> result [i] == # [trigger] arr1 [i] ^ # [trigger] arr2 [i] , { let mut result = Vec :: new () ; let mut idx = 0 ; while idx < arr1 . len () invariant result . len () == idx , idx <= arr1 . len () , arr1 . len () == arr2 . len () , forall | i : int | 0 <= i < idx && i < arr1 . len () && i < arr2 . len () ==> result [i] == arr1 [i] ^ arr2 [i] , decreases arr1 . len () - idx , { assert (idx < arr1 . len ()) ; assert (idx < arr2 . len ()) ; result . push (arr1 [idx] ^ arr2 [idx]) ; idx += 1 ; } result }

} // verus!