use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn copy (a : & Vec < int > , i : usize , acc : & mut Vec < int >) requires i <= a . len () , old (acc) . len () == i , forall | j : int | 0 <= j < i ==> old (acc) [j] == a [j] , ensures acc . len () == a . len () , forall | j : int | 0 <= j < a . len () ==> acc [j] == a [j] , decreases a . len () - i , { if i < a . len () { acc . push (a [i]) ; copy (a , i + 1 , acc) ; } }

} // verus!