use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
# [verifier :: loop_isolation (false)] fn append_with_element (a : & Vec < i32 > , b : i32) -> (result : Vec < i32 >) ensures result . len () == a . len () + 1 , forall | i : int | # ! [auto] 0 <= i && i < result . len () ==> result [i] == (if i < a . len () { a [i] } else { b }) , { let mut result : Vec < i32 > = Vec :: new () ; let mut i = 0 ; while i < a . len () invariant 0 <= i && i <= a . len () , result . len () == i , forall | j : int | 0 <= j && j < i ==> result [j] == a [j] , decreases a . len () - i { result . push (a [i]) ; i = i + 1 ; } result . push (b) ; result }

} // verus!