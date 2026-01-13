use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn test_array_elements (a : & mut Vec < i32 > , j : usize) requires 0 <= j < old (a) . len () , ensures a [j as int] == 60 , forall | k : int | 0 <= k < old (a) . len () && k != j ==> a [k] == old (a) [k] , { a . set (j , 60) ; }

} // verus!