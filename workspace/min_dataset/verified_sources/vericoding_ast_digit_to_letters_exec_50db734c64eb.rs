use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn digit_to_letters (c : char) -> Seq < char > { match c { '2' => seq ! ['a' , 'b' , 'c'] , '3' => seq ! ['d' , 'e' , 'f'] , '4' => seq ! ['g' , 'h' , 'i'] , '5' => seq ! ['j' , 'k' , 'l'] , '6' => seq ! ['m' , 'n' , 'o'] , '7' => seq ! ['p' , 'q' , 'r' , 's'] , '8' => seq ! ['t' , 'u' , 'v'] , '9' => seq ! ['w' , 'x' , 'y' , 'z'] , _ => seq ! [] , } }
fn digit_to_letters_exec (c : char) -> (result : Vec < char >) ensures result @ == digit_to_letters (c) { match c { '2' => vec ! ['a' , 'b' , 'c'] , '3' => vec ! ['d' , 'e' , 'f'] , '4' => vec ! ['g' , 'h' , 'i'] , '5' => vec ! ['j' , 'k' , 'l'] , '6' => vec ! ['m' , 'n' , 'o'] , '7' => vec ! ['p' , 'q' , 'r' , 's'] , '8' => vec ! ['t' , 'u' , 'v'] , '9' => vec ! ['w' , 'x' , 'y' , 'z'] , _ => Vec :: new () , } }

} // verus!