use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn contains (arr : & Vec < i32 > , key : i32) -> (result : bool) ensures result == (exists | i : int | 0 <= i < arr . len () && (arr [i] == key)) , { for i in 0 .. arr . len () invariant forall | j : int | 0 <= j < i ==> arr [j] != key , { if arr [i] == key { return true ; } } false }
fn intersection (arr1 : & Vec < i32 > , arr2 : & Vec < i32 >) -> (result : Vec < i32 >) ensures forall | i : int | 0 <= i < result . len () ==> (arr1 @ . contains (# [trigger] result [i]) && arr2 @ . contains (# [trigger] result [i] ,)) , forall | i : int , j : int | 0 <= i < j < result . len () ==> result [i] != result [j] , { let mut result = Vec :: new () ; for i in 0 .. arr1 . len () invariant forall | k : int | 0 <= k < result . len () ==> (arr1 @ . contains (# [trigger] result [k]) && arr2 @ . contains (# [trigger] result [k] ,)) , forall | k : int , j : int | 0 <= k < j < result . len () ==> result [k] != result [j] , { let elem = arr1 [i] ; if contains (arr2 , elem) && ! contains (& result , elem) { result . push (elem) ; } } result }

} // verus!