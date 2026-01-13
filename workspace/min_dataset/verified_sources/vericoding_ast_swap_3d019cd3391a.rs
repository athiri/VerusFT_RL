use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
# [verifier :: external_body] fn swap (a : & mut Vec < bool > , i : usize , j : usize) requires 0 <= i < j < old (a) . len () , ensures a [i as int] == old (a) [j as int] , a [j as int] == old (a) [i as int] , forall | k : int | 0 <= k < a . len () && k != i && k != j ==> a [k] == old (a) [k] , a . len () == old (a) . len () , a @ . to_multiset () =~~= old (a) @ . to_multiset () , { }

} // verus!