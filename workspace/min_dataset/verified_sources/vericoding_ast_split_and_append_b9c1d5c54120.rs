use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn split_and_append (list : & Vec < i32 > , n : usize) -> (new_list : Vec < i32 >) requires list @ . len () > 0 , 0 < n < list @ . len () , ensures new_list @ == list @ . subrange (n as int , list @ . len () as int) . add (list @ . subrange (0 , n as int)) , { let mut new_list = Vec :: new () ; let mut i = n ; while i < list . len () invariant 0 < n < list @ . len () , n <= i <= list @ . len () , new_list @ == list @ . subrange (n as int , i as int) , decreases list @ . len () - i , { new_list . push (list [i]) ; i += 1 ; } let mut j = 0 ; while j < n invariant 0 < n < list @ . len () , 0 <= j <= n , new_list @ == list @ . subrange (n as int , list @ . len () as int) . add (list @ . subrange (0 , j as int)) , decreases n - j , { new_list . push (list [j]) ; j += 1 ; } new_list }

} // verus!