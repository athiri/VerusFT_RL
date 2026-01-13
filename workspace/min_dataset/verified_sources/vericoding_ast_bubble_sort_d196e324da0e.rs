use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn bubble_sort_precond (a : Seq < i32 >) -> bool { true }
fn bubble_outer (a : & mut Vec < i32 > , i : usize) requires i + 1 < old (a) . len () , ensures a . len () == old (a) . len () , decreases i , { if i > 0 { bubble_inner (a , 0 , i) ; bubble_outer (a , i - 1) ; } }
fn bubble_inner (a : & mut Vec < i32 > , j : usize , i : usize) requires j <= i , i + 1 < old (a) . len () , ensures a . len () == old (a) . len () , decreases i - j , { if j < i { if a [j] > a [j + 1] { swap (a , j , j + 1) ; } bubble_inner (a , j + 1 , i) ; } }
fn swap (a : & mut Vec < i32 > , i : usize , j : usize) requires i < old (a) . len () , j < old (a) . len () , ensures a . len () == old (a) . len () , a [i as int] == old (a) [j as int] , a [j as int] == old (a) [i as int] , forall | k : int | 0 <= k < a . len () && k != i && k != j ==> a [k] == old (a) [k] , { let temp = a [i] ; a . set (i , a [j]) ; a . set (j , temp) ; }
fn bubble_sort (a : Vec < i32 >) -> (result : Vec < i32 >) requires bubble_sort_precond (a @) , ensures result . len () == a . len () , { let mut result = a ; if result . len () >= 2 { let len = result . len () ; bubble_outer (& mut result , len - 2) ; } result }

} // verus!