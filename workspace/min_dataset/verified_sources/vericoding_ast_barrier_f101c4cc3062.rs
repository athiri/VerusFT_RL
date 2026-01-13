use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn barrier (v : & [int] , p : usize) -> (b : bool) requires v . len () > 0 , p < v . len () , ensures b == (forall | k : int , l : int | 0 <= k <= p && p < l < v . len () ==> v [k] < v [l]) { let mut i = 0 ; if p >= v . len () - 1 { return true ; } while i <= p invariant 0 <= i <= p + 1 , i <= v . len () , p < v . len () - 1 , forall | k : int , l : int | 0 <= k < i && p < l < v . len () ==> v [k] < v [l] decreases p + 1 - i { let mut j = p + 1 ; while j < v . len () invariant 0 <= i <= p , i < v . len () , p + 1 <= j <= v . len () , p < v . len () - 1 , forall | l : int | p < l < j ==> v [i as int] < v [l] , forall | k : int , l : int | 0 <= k < i && p < l < v . len () ==> v [k] < v [l] decreases v . len () - j { if v [i] >= v [j] { return false ; } j += 1 ; } i += 1 ; } true }

} // verus!