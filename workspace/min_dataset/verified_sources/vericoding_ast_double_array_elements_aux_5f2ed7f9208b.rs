use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn double_array_elements_precond (s : Vec < i32 >) -> bool { forall | i : int | # ! [auto] 0 <= i < s . len () ==> s [i] <= MAX / 2 && s [i] >= MIN / 2 }
fn double_array_elements_aux (s_old : Vec < i32 > , s : Vec < i32 > , i : usize) -> (result : Vec < i32 >) requires s . len () == s_old . len () , i <= s . len () , double_array_elements_precond (s_old) , forall | j : int | # ! [auto] 0 <= j < i ==> s [j] == 2 * s_old [j] , forall | j : int | # ! [auto] i <= j < s . len () ==> s [j] == s_old [j] , ensures result . len () == s . len () , forall | j : int | # ! [auto] 0 <= j < s . len () ==> result [j] == 2 * s_old [j] , decreases s . len () - i { if i == s . len () { return s ; } else { let mut s_next = s ; s_next . set (i , 2 * s_old [i]) ; proof { assert (s_next . len () == s_old . len ()) ; assert (forall | j : int | # ! [auto] 0 <= j < i ==> s_next [j] == 2 * s_old [j]) ; assert (s_next [i as int] == 2 * s_old [i as int]) ; assert (forall | j : int | # ! [auto] 0 <= j < (i + 1) ==> s_next [j] == 2 * s_old [j]) ; assert (forall | j : int | # ! [auto] (i + 1) <= j < s_next . len () ==> s_next [j] == s_old [j]) ; } double_array_elements_aux (s_old , s_next , i + 1) } }

} // verus!