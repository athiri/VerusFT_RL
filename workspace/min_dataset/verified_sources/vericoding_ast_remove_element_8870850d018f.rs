use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
# [verifier :: loop_isolation (false)] fn remove_element (a : & [i32] , pos : usize) -> (result : Vec < i32 >) requires 0 <= pos < a . len () , ensures result . len () == a . len () - 1 , forall | i : int | 0 <= i < pos ==> result [i] == a [i] , forall | i : int | pos <= i < result . len () ==> result [i] == a [i + 1] , { let mut result = Vec :: new () ; let mut i = 0 ; while i < a . len () invariant i <= a . len () , result . len () == if i <= pos { i as int } else { i as int - 1 } , forall | j : int | 0 <= j < result . len () && j < pos ==> result [j] == a [j] , forall | j : int | pos <= j < result . len () ==> result [j] == a [j + 1] , decreases a . len () - i { if i != pos { result . push (a [i]) ; } i += 1 ; } result }

} // verus!