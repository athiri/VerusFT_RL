use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn all_digits_precond (s : Seq < char >) -> bool { true }
spec fn all_digits_postcond (s : Seq < char > , result : bool) -> bool { result == (forall | i : int | 0 <= i < s . len () ==> is_digit_spec (s [i])) }
spec fn is_digit_spec (c : char) -> bool { c >= '0' && c <= '9' }
fn is_digit (c : char) -> (result : bool) ensures result == is_digit_spec (c) , { c >= '0' && c <= '9' }
fn all_digits (s : & Vec < char >) -> (result : bool) requires all_digits_precond (s @) , ensures all_digits_postcond (s @ , result) , { let mut i = 0 ; while i < s . len () invariant 0 <= i <= s . len () , forall | j : int | 0 <= j < i ==> is_digit_spec (s @ [j]) , decreases s . len () - i { if ! is_digit (s [i]) { return false ; } i += 1 ; } true }

} // verus!