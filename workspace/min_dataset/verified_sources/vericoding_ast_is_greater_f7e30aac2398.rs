use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn is_greater (n : i32 , a : & Vec < i32 >) -> (result : bool) requires a . len () > 0 ensures result <==> (forall | i : int | 0 <= i < a . len () ==> n > a [i]) { for i in 0 .. a . len () invariant forall | j : int | 0 <= j < i ==> n > a [j] { if n <= a [i] { return false ; } } true }

} // verus!