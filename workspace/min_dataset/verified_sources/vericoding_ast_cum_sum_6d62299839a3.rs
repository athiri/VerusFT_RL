use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn cum_sum (a : & Vec < int >) -> (res : Vec < int >) ensures res . len () == a . len () , a . len () > 0 ==> res [0] == a [0] , forall | i : int | 1 <= i < a . len () ==> res [i] == res [i - 1] + a [i] , { let mut res : Vec < int > = Vec :: new () ; for i in 0 .. a . len () invariant res . len () == i , forall | j : int | 0 <= j < i ==> # [trigger] res [j] == if j == 0 { a [0] } else { res [j - 1] + a [j] } , { if i == 0 { res . push (a [i]) ; } else { let prev_val = res [i - 1] ; res . push (prev_val + a [i]) ; } } res }

} // verus!