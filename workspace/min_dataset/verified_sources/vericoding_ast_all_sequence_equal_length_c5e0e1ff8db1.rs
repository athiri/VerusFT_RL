use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn all_sequence_equal_length (seq : & Vec < Vec < i32 > >) -> (result : bool) requires seq . len () > 0 , ensures result == (forall | i : int , j : int | (0 <= i < seq . len () && 0 <= j < seq . len ()) ==> (# [trigger] seq [i] . len () == # [trigger] seq [j] . len ())) , { let first_len = seq [0] . len () ; for k in 1 .. seq . len () invariant forall | i : int | (0 <= i < k) ==> seq [i] . len () == first_len , { if seq [k] . len () != first_len { assert (seq [k as int] . len () != seq [0] . len ()) ; return false ; } } assert (forall | i : int | (0 <= i < seq . len ()) ==> seq [i] . len () == first_len) ; true }

} // verus!