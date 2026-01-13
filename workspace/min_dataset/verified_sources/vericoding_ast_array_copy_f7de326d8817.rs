use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
# [verifier :: loop_isolation (false)] fn array_copy (a : Vec < i32 >) -> (result : Vec < i32 >) ensures result . len () == a . len () , forall | i : int | 0 <= i && i < a . len () ==> result [i] == a [i] , { let mut result = Vec :: new () ; let mut i = 0 ; while i < a . len () invariant i <= a . len () , result . len () == i , forall | j : int | 0 <= j && j < i ==> result [j] == a [j] , decreases a . len () - i , { result . push (a [i]) ; i += 1 ; } result }

} // verus!