use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn inner_expr_replace_blanks_with_chars (str1 : & Vec < char > , ch : char , i : int) -> (result : char) { if str1 [i] == ' ' { ch } else { str1 [i] } }
fn replace_blanks_with_chars (str1 : & Vec < char > , ch : char) -> (result : Vec < char >) ensures str1 @ . len () == result @ . len () , forall | i : int | 0 <= i < str1 . len () ==> result [i] == inner_expr_replace_blanks_with_chars (str1 , ch , i) , { let mut result = Vec :: new () ; let mut idx = 0 ; while idx < str1 . len () invariant idx <= str1 . len () , result @ . len () == idx , forall | i : int | 0 <= i < idx ==> result [i] == inner_expr_replace_blanks_with_chars (str1 , ch , i) , decreases str1 . len () - idx , { let new_char = if str1 [idx] == ' ' { ch } else { str1 [idx] } ; result . push (new_char) ; idx += 1 ; } result }

} // verus!