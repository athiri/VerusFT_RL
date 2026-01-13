use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn to_toggle_case_spec (s : char) -> (result : char) { if is_lower_case (s) { shift_minus_32_spec (s) } else if is_upper_case (s) { shift32_spec (s) } else { s } }
spec fn shift_minus_32_spec (c : char) -> (result : char) { ((c as u8) - 32) as char }
spec fn shift32_spec (c : char) -> (result : char) { ((c as u8) + 32) as char }
spec fn is_upper_case (c : char) -> (result : bool) { c >= 'A' && c <= 'Z' }
spec fn is_lower_case (c : char) -> (result : bool) { c >= 'a' && c <= 'z' }
fn to_toggle_case (str1 : & Vec < char >) -> (toggle_case : Vec < char >) ensures str1 @ . len () == toggle_case @ . len () , forall | i : int | 0 <= i < str1 . len () ==> toggle_case [i] == to_toggle_case_spec (# [trigger] str1 [i]) , { let mut result = Vec :: new () ; let mut idx = 0 ; while idx < str1 . len () invariant idx <= str1 . len () , result . len () == idx , forall | i : int | 0 <= i < idx ==> result [i] == to_toggle_case_spec (str1 [i]) , decreases str1 . len () - idx , { let c = str1 [idx] ; let toggled_char = if c >= 'a' && c <= 'z' { ((c as u8) - 32) as char } else if c >= 'A' && c <= 'Z' { ((c as u8) + 32) as char } else { c } ; result . push (toggled_char) ; idx += 1 ; } result }

} // verus!