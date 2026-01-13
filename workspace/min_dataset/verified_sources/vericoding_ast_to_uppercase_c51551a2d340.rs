use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn inner_expr_to_uppercase (str1 : & Vec < char > , i : int) -> (result : char) { if is_lower_case (# [trigger] str1 [i]) { shift_minus_32_spec (str1 [i]) } else { str1 [i] } }
spec fn is_lower_case (c : char) -> (result : bool) { c >= 'a' && c <= 'z' }
spec fn shift_minus_32_spec (c : char) -> (result : char) { ((c as u8) - 32) as char }
fn to_uppercase (str1 : & Vec < char >) -> (result : Vec < char >) ensures str1 @ . len () == result @ . len () , forall | i : int | 0 <= i < str1 . len () ==> (result [i] == (inner_expr_to_uppercase (str1 , i))) , { let mut result = Vec :: new () ; let mut idx = 0 ; while idx < str1 . len () invariant idx <= str1 . len () , result . len () == idx , forall | i : int | 0 <= i < idx ==> result [i] == inner_expr_to_uppercase (str1 , i) , decreases str1 . len () - idx { let c = str1 [idx] ; if c >= 'a' && c <= 'z' { let uppercase_c = ((c as u8) - 32) as char ; result . push (uppercase_c) ; } else { result . push (c) ; } idx += 1 ; } result }

} // verus!