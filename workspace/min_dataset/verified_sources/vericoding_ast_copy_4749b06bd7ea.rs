use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn copy (a : & Vec < u64 >) -> (b : Vec < u64 >) requires a . len () <= 100 , ensures b @ . len () == a @ . len () , forall | i : int | (0 <= i && i < a . len ()) ==> b [i] == a [i] , { let mut b = Vec :: new () ; let mut i = 0 ; while i < a . len () invariant i <= a . len () , b @ . len () == i , forall | j : int | (0 <= j && j < i) ==> b [j] == a [j] , decreases a . len () - i { b . push (a [i]) ; i += 1 ; } b }

} // verus!