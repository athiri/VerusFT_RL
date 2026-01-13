use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn linear_search_postcond (a : & Vec < i32 > , e : i32 , result : usize) -> bool { result < a . len () && a [result as int] == e && forall | k : int | 0 <= k < result ==> a [k] != e }
spec fn linear_search_precond (a : & Vec < i32 > , e : i32) -> bool { exists | i : int | 0 <= i < a . len () && a [i as int] == e }
fn linear_search_aux (a : & Vec < i32 > , e : i32 , n : usize) -> (result : usize) requires n <= a . len () , linear_search_precond (a , e) , forall | k : int | 0 <= k < n ==> a [k] != e , ensures linear_search_postcond (a , e , result) , decreases a . len () - n , { if n < a . len () { if a [n] == e { n } else { linear_search_aux (a , e , n + 1) } } else { 0 } }
fn linear_search (a : & Vec < i32 > , e : i32) -> (result : usize) requires linear_search_precond (a , e) , ensures linear_search_postcond (a , e , result) , { linear_search_aux (a , e , 0) }

} // verus!