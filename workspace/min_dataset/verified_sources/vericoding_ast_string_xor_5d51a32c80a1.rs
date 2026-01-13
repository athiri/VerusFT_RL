use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn is_binary_digit (c : char) -> (ret : bool) { c == '0' || c == '1' }
spec fn xor_char (a : char , b : char) -> (result : char) recommends is_binary_digit (a) , is_binary_digit (b) , { if a == b { '0' } else { '1' } }
fn string_xor (a : & [char] , b : & [char]) -> (result : Vec < char >) requires a @ . len () == b @ . len () , forall | i : int | 0 <= i < a @ . len () as int ==> is_binary_digit (# [trigger] a [i]) , forall | i : int | 0 <= i < b @ . len () as int ==> is_binary_digit (# [trigger] b [i]) , ensures result . len () == a @ . len () , forall | i : int | 0 <= i < result . len () as int ==> # [trigger] result [i] == xor_char (a [i] , b [i]) , { let mut result = Vec :: new () ; let mut i = 0 ; while i < a . len () invariant 0 <= i <= a @ . len () , a @ . len () == b @ . len () , result . len () == i , forall | j : int | 0 <= j < i ==> # [trigger] result [j] == xor_char (a [j] , b [j]) , decreases a @ . len () - i { let xor_result = if a [i] == b [i] { '0' } else { '1' } ; result . push (xor_result) ; i = i + 1 ; } result }

} // verus!