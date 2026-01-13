use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn find_first_odd_postcond (a : & Vec < i32 > , result : Option < usize >) -> bool { match result { Some (idx) => { &&& idx < a . len () &&& is_odd (a [idx as int] as int) &&& forall | j : int | 0 <= j < idx ==> ! is_odd (a [j] as int) } None => forall | i : int | 0 <= i < a . len () ==> ! is_odd (a [i] as int) } }
spec fn is_odd (x : int) -> bool { x % 2 != 0 }
spec fn find_first_odd_precond (a : & Vec < i32 >) -> bool { a . len () > 0 }
fn find_first_odd (a : & Vec < i32 >) -> (result : Option < usize >) requires find_first_odd_precond (a) ensures find_first_odd_postcond (a , result) { let mut i : usize = 0 ; while i < a . len () invariant i <= a . len () , forall | j : int | 0 <= j < i ==> ! is_odd (a [j] as int) decreases a . len () - i { if a [i] % 2 != 0 { proof { assert (is_odd (a [i as int] as int)) ; } return Some (i) ; } i += 1 ; } None }

} // verus!