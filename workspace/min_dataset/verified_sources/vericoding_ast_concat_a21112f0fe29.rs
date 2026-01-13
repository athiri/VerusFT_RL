use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn concat_precond (a : & Vec < i32 > , b : & Vec < i32 >) -> bool { true }
spec fn concat_postcond (a : & Vec < i32 > , b : & Vec < i32 > , result : & Vec < i32 >) -> bool { result . len () == a . len () + b . len () && (forall | k : int | 0 <= k < a . len () ==> result [k] == a [k]) && (forall | k : int | 0 <= k < b . len () ==> result [k + a . len ()] == b [k]) }
fn concat (a : & Vec < i32 > , b : & Vec < i32 >) -> (result : Vec < i32 >) requires concat_precond (a , b) , a . len () + b . len () <= MAX , ensures concat_postcond (a , b , & result) , { let mut result = Vec :: new () ; let mut i = 0 ; while i < a . len () invariant i <= a . len () , result . len () == i , forall | k : int | 0 <= k < i ==> result [k] == a [k] , decreases a . len () - i , { result . push (a [i]) ; i += 1 ; } let mut j = 0 ; while j < b . len () invariant j <= b . len () , result . len () == a . len () + j , forall | k : int | 0 <= k < a . len () ==> result [k] == a [k] , forall | k : int | 0 <= k < j ==> result [k + a . len ()] == b [k] , decreases b . len () - j , { result . push (b [j]) ; j += 1 ; } result }

} // verus!