use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn swap (a : & mut Vec < i32 > , i : usize , j : usize) requires i < old (a) . len () , j < old (a) . len () , ensures a . len () == old (a) . len () , a [i as int] == old (a) [j as int] , a [j as int] == old (a) [i as int] , forall | k : int | 0 <= k < a . len () && k != i && k != j ==> a [k] == old (a) [k] , { let temp = a [i] ; let temp_j = a [j] ; a . set (i , temp_j) ; a . set (j , temp) ; }

} // verus!