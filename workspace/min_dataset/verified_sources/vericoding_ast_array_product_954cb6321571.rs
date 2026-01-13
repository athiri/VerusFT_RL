use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
# [verifier :: loop_isolation (false)] fn array_product (a : Vec < i32 > , b : Vec < i32 >) -> (result : Vec < i64 >) by (nonlinear_arith) requires a . len () == b . len () , ensures result . len () == a . len () , forall | i : int | # ! [auto] 0 <= i && i < a . len () ==> result [i] == (a [i] as i64) * (b [i] as i64) , { let mut result = Vec :: new () ; let mut i = 0 ; while i < a . len () invariant result . len () == i , i <= a . len () , forall | j : int | # ! [auto] 0 <= j && j < i ==> result [j] == (a [j] as i64) * (b [j] as i64) , decreases a . len () - i , { let product = (a [i] as i64) * (b [i] as i64) ; result . push (product) ; i += 1 ; } result }

} // verus!