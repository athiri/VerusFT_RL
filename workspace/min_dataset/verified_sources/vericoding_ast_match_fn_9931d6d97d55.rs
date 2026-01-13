use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn match_precond (s : Seq < char > , p : Seq < char >) -> bool { s . len () == p . len () }
fn match_fn (s : Vec < char > , p : Vec < char >) -> (result : bool) requires match_precond (s @ , p @) ensures result <==> forall | n : int | 0 <= n < s . len () ==> (s [n] == p [n] || p [n] == '?') { let mut i = 0 ; while i < s . len () invariant 0 <= i <= s . len () , s @ . len () == p @ . len () , forall | j : int | 0 <= j < i ==> (s [j] == p [j] || p [j] == '?') decreases s . len () - i { if s [i] != p [i] && p [i] != '?' { return false ; } i += 1 ; } true }

} // verus!