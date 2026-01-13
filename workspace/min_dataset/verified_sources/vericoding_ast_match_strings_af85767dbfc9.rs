use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn match_strings (s : Vec < char > , p : Vec < char >) -> (b : bool) requires s . len () == p . len () , ensures b <==> forall | n : int | 0 <= n < s . len () ==> s [n] == p [n] || p [n] == '?' { let mut i = 0 ; while i < s . len () invariant 0 <= i <= s . len () , s . len () == p . len () , forall | n : int | 0 <= n < i ==> s [n] == p [n] || p [n] == '?' decreases s . len () - i { if s [i] != p [i] && p [i] != '?' { return false ; } i += 1 ; } return true ; }

} // verus!