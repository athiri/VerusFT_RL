use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn to_array_postcond (xs : Seq < int > , result : Seq < int >) -> bool { result . len () == xs . len () && (forall | i : int | 0 <= i < xs . len () ==> result [i] == xs [i]) }
spec fn to_array_precond (xs : Seq < int >) -> bool { true }
fn to_array (xs : Vec < int >) -> (result : Vec < int >) requires to_array_precond (xs @) , ensures to_array_postcond (xs @ , result @) , { let mut result = Vec :: new () ; let mut i = 0 ; while i < xs . len () invariant 0 <= i <= xs . len () , result . len () == i , forall | j : int | 0 <= j < i ==> result @ [j] == xs @ [j] , decreases xs . len () - i , { result . push (xs [i]) ; i += 1 ; } result }

} // verus!