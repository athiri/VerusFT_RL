use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn copy_from (a : & Vec < i32 > , i : usize , acc : & mut Vec < i32 >) requires i <= a . len () , old (acc) . len () + (a . len () - i) <= MAX , ensures acc . len () == old (acc) . len () + (a . len () - i) , forall | j : int | 0 <= j < old (acc) . len () ==> acc [j] == old (acc) [j] , forall | j : int | old (acc) . len () <= j < acc . len () ==> acc [j] == a [(j - old (acc) . len () + i) as int] , decreases a . len () - i { if i < a . len () { acc . push (a [i]) ; copy_from (a , i + 1 , acc) ; } }

} // verus!