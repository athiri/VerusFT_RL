use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn split_array (list : & Vec < i32 > , l : usize) -> (new_list : (Vec < i32 > , Vec < i32 >)) requires list @ . len () > 0 , 0 < l < list @ . len () , ensures new_list . 0 @ == list @ . subrange (0 , l as int) , new_list . 1 @ == list @ . subrange (l as int , list . len () as int) , { let mut first_part : Vec < i32 > = Vec :: new () ; let mut second_part : Vec < i32 > = Vec :: new () ; let mut i = 0 ; while i < l invariant 0 <= i <= l , l < list . len () , first_part @ == list @ . subrange (0 , i as int) , decreases l - i , { first_part . push (list [i]) ; i += 1 ; } let mut j = l ; while j < list . len () invariant l <= j <= list . len () , first_part @ == list @ . subrange (0 , l as int) , second_part @ == list @ . subrange (l as int , j as int) , decreases list . len () - j , { second_part . push (list [j]) ; j += 1 ; } (first_part , second_part) }

} // verus!