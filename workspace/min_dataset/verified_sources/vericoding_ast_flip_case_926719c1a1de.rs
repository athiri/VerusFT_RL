use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn flip_case_spec (c : char) -> (result : char) { if is_lower_case (c) { shift_minus_32_spec (c) } else if is_upper_case (c) { shift_plus_32_spec (c) } else { c } }
spec fn shift_minus_32_spec (c : char) -> (result : char) { ((c as u8) - 32) as char }
spec fn shift_plus_32_spec (c : char) -> (result : char) { ((c as u8) + 32) as char }
spec fn is_upper_case (c : char) -> (result : bool) { c >= 'A' && c <= 'Z' }
spec fn is_lower_case (c : char) -> (result : bool) { c >= 'a' && c <= 'z' }
fn flip_case (str : & [char]) -> (flipped_case : Vec < char >) ensures str @ . len () == flipped_case @ . len () , forall | i : int | 0 <= i < str . len () ==> flipped_case [i] == flip_case_spec (# [trigger] str [i]) , { let mut result = Vec :: new () ; let mut idx = 0 ; while idx < str . len () invariant idx <= str . len () , result . len () == idx , forall | i : int | 0 <= i < idx ==> result [i] == flip_case_spec (str [i]) , decreases str . len () - idx , { let c = str [idx] ; let flipped_c = if c >= 'a' && c <= 'z' { ((c as u8) - 32) as char } else if c >= 'A' && c <= 'Z' { ((c as u8) + 32) as char } else { c } ; result . push (flipped_c) ; idx += 1 ; } result }

} // verus!