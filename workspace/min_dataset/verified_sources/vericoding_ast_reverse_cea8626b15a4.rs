use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn reverse (v : & mut Vec < u64 >) ensures v . len () == old (v) . len () , forall | i : int | 0 <= i < old (v) . len () ==> v [i] == old (v) [old (v) . len () - i - 1] , { let len = v . len () ; let mut i = 0 ; while i < len / 2 invariant v . len () == len , len == old (v) . len () , forall | j : int | 0 <= j < i ==> v [j] == old (v) [len - j - 1] , forall | j : int | len - i <= j < len ==> v [j] == old (v) [len - j - 1] , forall | j : int | i <= j < len - i ==> v [j] == old (v) [j] , decreases len / 2 - i , { let temp = v [i] ; let temp2 = v [len - i - 1] ; v . set (i , temp2) ; v . set (len - i - 1 , temp) ; i += 1 ; } }

} // verus!