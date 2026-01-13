use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn decode_char_spec (c : int) -> (result : int) recommends 65 <= c <= 90 , { (c - 65 + 26 - 5) % 26 + 65 }
fn decode_char (c : u8) -> (r : u8) requires 65 <= c <= 90 , ensures r == decode_char_spec (c as int) , 65 <= r <= 90 , { let shifted = (c - 65 + 21) % 26 + 65 ; shifted }

} // verus!