use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn single_digit_number_to_char (n : nat) -> (result : char) { if n == 0 { '0' } else if n == 1 { '1' } else if n == 2 { '2' } else if n == 3 { '3' } else if n == 4 { '4' } else if n == 5 { '5' } else if n == 6 { '6' } else if n == 7 { '7' } else if n == 8 { '8' } else { '9' } }
fn single_digit_number_to_char_impl (n : u8) -> (output : char) requires 0 <= n <= 9 , ensures single_digit_number_to_char (n as nat) == output , { match n { 0 => '0' , 1 => '1' , 2 => '2' , 3 => '3' , 4 => '4' , 5 => '5' , 6 => '6' , 7 => '7' , 8 => '8' , _ => '9' , } }

} // verus!