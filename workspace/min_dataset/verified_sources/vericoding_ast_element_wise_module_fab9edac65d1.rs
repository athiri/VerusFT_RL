use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn element_wise_module (arr1 : & Vec < u32 > , arr2 : & Vec < u32 >) -> (result : Vec < u32 >) requires arr1 . len () == arr2 . len () , forall | i : int | 0 <= i < arr2 . len () ==> arr2 [i] != 0 , forall | i : int | (0 <= i < arr1 . len ()) ==> (MIN <= # [trigger] (arr1 [i] % arr2 [i]) <= MAX) , ensures result @ . len () == arr1 @ . len () , forall | i : int | 0 <= i < result . len () ==> # [trigger] result [i] == # [trigger] (arr1 [i] % arr2 [i]) , { let mut result = Vec :: new () ; let mut i = 0 ; while i < arr1 . len () invariant 0 <= i <= arr1 . len () , arr1 . len () == arr2 . len () , result @ . len () == i , forall | j : int | 0 <= j < i ==> result [j] == arr1 [j] % arr2 [j] , forall | k : int | 0 <= k < arr2 . len () ==> arr2 [k] != 0 , forall | k : int | (0 <= k < arr1 . len ()) ==> (MIN <= # [trigger] (arr1 [k] % arr2 [k]) <= MAX) , decreases arr1 . len () - i { let mod_result = arr1 [i] % arr2 [i] ; result . push (mod_result) ; i += 1 ; } result }

} // verus!