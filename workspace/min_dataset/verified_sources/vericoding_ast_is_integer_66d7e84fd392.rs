use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn is_digit_sepc (c : char) -> (res : bool) { (c as u32) >= 48 && (c as u32) <= 57 }
fn is_digit (c : char) -> (res : bool) ensures res == is_digit_sepc (c) , { let code = c as u32 ; code >= 48 && code <= 57 }
fn is_integer (text : & Vec < char >) -> (result : bool) ensures result == (forall | i : int | 0 <= i < text . len () ==> (# [trigger] is_digit_sepc (text [i]))) , { let mut i = 0 ; while i < text . len () invariant 0 <= i <= text . len () , forall | j : int | 0 <= j < i ==> is_digit_sepc (text [j]) , decreases text . len () - i { if ! is_digit (text [i]) { return false ; } i += 1 ; } true }

} // verus!