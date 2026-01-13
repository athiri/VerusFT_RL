use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn element_wise_multiplication (arr1 : & Vec < i32 > , arr2 : & Vec < i32 >) -> (result : Vec < i32 >) requires arr1 . len () == arr2 . len () , forall | i : int | (0 <= i < arr1 . len ()) ==> (MIN <= # [trigger] (arr1 [i] * arr2 [i]) <= MAX) , ensures result . len () == arr1 . len () , forall | i : int | 0 <= i < result . len () ==> # [trigger] result [i] == # [trigger] (arr1 [i] * arr2 [i]) , { let mut result = Vec :: new () ; let mut i = 0 ; while i < arr1 . len () invariant 0 <= i <= arr1 . len () , result . len () == i , arr1 . len () == arr2 . len () , forall | j : int | 0 <= j < i ==> result [j as int] == arr1 [j] * arr2 [j] , forall | j : int | (0 <= j < arr1 . len ()) ==> (MIN <= # [trigger] (arr1 [j] * arr2 [j]) <= MAX) , decreases arr1 . len () - i , { assert (i < arr1 . len ()) ; assert (i < arr2 . len ()) ; assert ((i as int) < arr1 . len ()) ; assert ((i as int) < arr2 . len ()) ; assert (MIN <= arr1 [i as int] * arr2 [i as int] <= MAX) ; let product = arr1 [i] * arr2 [i] ; result . push (product) ; i += 1 ; } result }

} // verus!