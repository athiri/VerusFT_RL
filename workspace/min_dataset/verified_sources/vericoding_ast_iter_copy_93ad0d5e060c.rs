use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn iter_copy_precond (s : Seq < int >) -> bool { true }
fn iter_copy (s : & Vec < int >) -> (result : Vec < int >) requires iter_copy_precond (s @) , ensures s @ . len () == result @ . len () , forall | i : int | 0 <= i < s @ . len () ==> s @ [i] == result @ [i] , { let mut result = Vec :: new () ; let mut i = 0 ; while i < s . len () invariant 0 <= i <= s . len () , result . len () == i , forall | j : int | 0 <= j < i ==> s @ [j] == result @ [j] , decreases s . len () - i , { result . push (s [i]) ; i += 1 ; } result }

} // verus!