use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn digit_to_letters (c : char) -> Seq < char > { match c { '2' => seq ! ['a' , 'b' , 'c'] , '3' => seq ! ['d' , 'e' , 'f'] , '4' => seq ! ['g' , 'h' , 'i'] , '5' => seq ! ['j' , 'k' , 'l'] , '6' => seq ! ['m' , 'n' , 'o'] , '7' => seq ! ['p' , 'q' , 'r' , 's'] , '8' => seq ! ['t' , 'u' , 'v'] , '9' => seq ! ['w' , 'x' , 'y' , 'z'] , _ => seq ! [] , } }
fn digit_to_letters_exec (c : char) -> (result : Vec < char >) ensures result @ == digit_to_letters (c) { match c { '2' => vec ! ['a' , 'b' , 'c'] , '3' => vec ! ['d' , 'e' , 'f'] , '4' => vec ! ['g' , 'h' , 'i'] , '5' => vec ! ['j' , 'k' , 'l'] , '6' => vec ! ['m' , 'n' , 'o'] , '7' => vec ! ['p' , 'q' , 'r' , 's'] , '8' => vec ! ['t' , 'u' , 'v'] , '9' => vec ! ['w' , 'x' , 'y' , 'z'] , _ => Vec :: new () , } }
fn go (chars : & Vec < char > , start : usize) -> (result : Vec < Vec < char > >) requires start <= chars . len () decreases chars . len () - start { if start == chars . len () { let mut result = Vec :: new () ; result . push (Vec :: new ()) ; return result ; } let current_letters = digit_to_letters_exec (chars [start]) ; let rest_combinations = go (chars , start + 1) ; let mut result = Vec :: new () ; for i in 0 .. current_letters . len () invariant 0 <= i <= current_letters . len () { let letter = current_letters [i] ; for j in 0 .. rest_combinations . len () invariant 0 <= j <= rest_combinations . len () , 0 <= i < current_letters . len () { let mut combination = Vec :: new () ; combination . push (letter) ; let rest = & rest_combinations [j] ; for k in 0 .. rest . len () invariant 0 <= k <= rest . len () , combination . len () == k + 1 { combination . push (rest [k]) ; } result . push (combination) ; } } result }

} // verus!