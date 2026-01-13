use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn inner_expr_replace_with_colon (str1 : & Vec < char > , k : int) -> (result : char) { if is_space_comma_dot_spec (str1 [k]) { ':' } else { str1 [k] } }
spec fn is_space_comma_dot_spec (c : char) -> (result : bool) { (c == ' ') || (c == ',') || (c == '.') }
fn is_space_comma_dot (c : char) -> (result : bool) ensures result == is_space_comma_dot_spec (c) { (c == ' ') || (c == ',') || (c == '.') }
fn replace_with_colon (str1 : & Vec < char >) -> (result : Vec < char >) ensures str1 @ . len () == result @ . len () , forall | k : int | 0 <= k < result . len () ==> # [trigger] result [k] == inner_expr_replace_with_colon (str1 , k) , { let mut result = Vec :: new () ; let mut i = 0 ; while i < str1 . len () invariant 0 <= i <= str1 . len () , result . len () == i , forall | k : int | 0 <= k < i ==> # [trigger] result [k] == inner_expr_replace_with_colon (str1 , k) , decreases str1 . len () - i { let c = str1 [i] ; if is_space_comma_dot (c) { result . push (':') ; } else { result . push (c) ; } i += 1 ; } result }

} // verus!