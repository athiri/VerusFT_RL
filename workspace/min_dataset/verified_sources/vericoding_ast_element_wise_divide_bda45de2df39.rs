use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn element_wise_divide (arr1 : & Vec < u32 > , arr2 : & Vec < u32 >) -> (result : Vec < u32 >) requires arr1 . len () == arr2 . len () , forall | i : int | 0 <= i < arr2 . len () ==> arr2 [i] != 0 , forall | i : int | (0 <= i < arr1 . len ()) ==> (MIN <= # [trigger] (arr1 [i] / arr2 [i]) <= MAX) , ensures result @ . len () == arr1 @ . len () , forall | i : int | 0 <= i < result . len () ==> # [trigger] result [i] == # [trigger] (arr1 [i] / arr2 [i]) , { let mut result = Vec :: new () ; let mut i = 0 ; while i < arr1 . len () invariant i <= arr1 . len () , result @ . len () == i , forall | j : int | 0 <= j < i ==> result [j] == arr1 [j] / arr2 [j] , arr1 . len () == arr2 . len () , forall | j : int | 0 <= j < arr2 . len () ==> arr2 [j] != 0 , decreases arr1 . len () - i , { let quotient = arr1 [i] / arr2 [i] ; result . push (quotient) ; i += 1 ; } result }

} // verus!