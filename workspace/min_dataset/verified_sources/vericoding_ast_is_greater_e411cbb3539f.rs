use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn is_greater (n : i32 , a : & Vec < i32 >) -> (result : bool) requires a . len () > 0 ensures result <==> (forall | i : int | 0 <= i < a . len () ==> n > a [i]) { let mut i = 0 ; while i < a . len () invariant 0 <= i <= a . len () , forall | j : int | 0 <= j < i ==> n > a [j] decreases a . len () - i { if n <= a [i] { return false ; } i += 1 ; } true }

} // verus!