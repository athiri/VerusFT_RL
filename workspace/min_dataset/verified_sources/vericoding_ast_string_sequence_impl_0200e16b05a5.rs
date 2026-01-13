use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn string_sequence (n : nat) -> (result : Seq < char >) decreases n , { if n == 0 { seq ! ['0'] } else { string_sequence ((n - 1) as nat) . add (seq ! [' '] . add (number_to_char (n))) } }
spec fn number_to_char (n : nat) -> (result : Seq < char >) decreases n , { if (n == 0) { seq ! [] } else { number_to_char (n / 10) . add (seq ! [single_digit_number_to_char (n % 10)]) } }
spec fn single_digit_number_to_char (n : nat) -> (result : char) { if n == 0 { '0' } else if n == 1 { '1' } else if n == 2 { '2' } else if n == 3 { '3' } else if n == 4 { '4' } else if n == 5 { '5' } else if n == 6 { '6' } else if n == 7 { '7' } else if n == 8 { '8' } else { '9' } }
fn number_to_char_impl (n : u8) -> (char_vec : Vec < char >) ensures char_vec @ == number_to_char (n as nat) , decreases n { if n == 0 { Vec :: new () } else { let mut result = number_to_char_impl (n / 10) ; let digit_char = single_digit_number_to_char_impl (n % 10) ; result . push (digit_char) ; result } }
fn single_digit_number_to_char_impl (n : u8) -> (output : char) requires 0 <= n <= 9 , ensures single_digit_number_to_char (n as nat) == output , { match n { 0 => '0' , 1 => '1' , 2 => '2' , 3 => '3' , 4 => '4' , 5 => '5' , 6 => '6' , 7 => '7' , 8 => '8' , _ => '9' , } }
fn string_sequence_impl (n : u8) -> (string_seq : Vec < char >) ensures string_seq @ == string_sequence (n as nat) , decreases n { if n == 0 { vec ! ['0'] } else { let mut result = string_sequence_impl (n - 1) ; result . push (' ') ; let mut num_chars = number_to_char_impl (n) ; result . append (& mut num_chars) ; result } }

} // verus!