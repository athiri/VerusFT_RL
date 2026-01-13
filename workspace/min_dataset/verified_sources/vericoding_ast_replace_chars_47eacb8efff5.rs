use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn inner_epxr_replace_chars (str1 : & Vec < char > , old_char : char , new_char : char , i : int) -> (result : char) { if str1 @ [i] == old_char { new_char } else { str1 @ [i] } }
fn replace_chars (str1 : & Vec < char > , old_char : char , new_char : char) -> (result : Vec < char >) ensures str1 @ . len () == result @ . len () , forall | i : int | 0 <= i < str1 . len () ==> result @ [i] == inner_epxr_replace_chars (str1 , old_char , new_char , i) , { let mut result = Vec :: new () ; let mut j = 0 ; while j < str1 . len () invariant 0 <= j <= str1 . len () , result . len () == j , forall | i : int | 0 <= i < j ==> result @ [i] == inner_epxr_replace_chars (str1 , old_char , new_char , i) , decreases str1 . len () - j , { let ch = if str1 [j] == old_char { new_char } else { str1 [j] } ; result . push (ch) ; j += 1 ; } result }

} // verus!