use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn double_array_elements_precond (s : Vec < i32 >) -> bool { forall | i : int | # ! [auto] 0 <= i < s . len () ==> s [i] <= MAX / 2 && s [i] >= MIN / 2 }
spec fn double_array_elements_postcond (s : Vec < i32 > , result : Vec < i32 >) -> bool { result . len () == s . len () && forall | i : int | # ! [auto] 0 <= i < s . len () ==> result [i] == 2 * s [i] }
fn double_array_elements_aux (s_old : Vec < i32 > , s : Vec < i32 > , i : usize) -> (result : Vec < i32 >) requires s . len () == s_old . len () , i <= s . len () , double_array_elements_precond (s_old) , forall | j : int | # ! [auto] 0 <= j < i ==> s [j] == 2 * s_old [j] , forall | j : int | # ! [auto] i <= j < s . len () ==> s [j] == s_old [j] , ensures result . len () == s . len () , forall | j : int | # ! [auto] 0 <= j < s . len () ==> result [j] == 2 * s_old [j] , decreases s . len () - i { if i == s . len () { return s ; } else { let mut s_new = s ; s_new . set (i , 2 * s_old [i]) ; double_array_elements_aux (s_old , s_new , i + 1) } }
fn double_array_elements (s : Vec < i32 >) -> (result : Vec < i32 >) requires double_array_elements_precond (s) , ensures double_array_elements_postcond (s , result) , { let s_copy = s . clone () ; double_array_elements_aux (s , s_copy , 0) }

} // verus!