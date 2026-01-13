use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn remove_kth_element (list : & Vec < i32 > , k : usize) -> (new_list : Vec < i32 >) requires list . len () > 0 , 0 < k < list @ . len () , ensures new_list @ == list @ . subrange (0 , k - 1 as int) . add (list @ . subrange (k as int , list . len () as int) ,) , { let mut new_list = Vec :: new () ; assert (k > 0) ; assert (k < list . len ()) ; assert (k - 1 < list . len ()) ; let mut i = 0 ; while i < k - 1 invariant 0 <= i <= k - 1 , k - 1 < list . len () , i <= list . len () , new_list @ == list @ . subrange (0 , i as int) , decreases k - 1 - i , { assert (i < k - 1) ; assert (i < list . len ()) ; new_list . push (list [i]) ; i += 1 ; } assert (i == k - 1) ; assert (k - 1 < list . len ()) ; let mut j = k ; while j < list . len () invariant k <= j <= list . len () , k < list . len () , k - 1 < list . len () , new_list @ == list @ . subrange (0 , k - 1 as int) . add (list @ . subrange (k as int , j as int)) , decreases list . len () - j , { assert (j < list . len ()) ; new_list . push (list [j]) ; j += 1 ; } assert (j == list . len ()) ; assert (new_list @ == list @ . subrange (0 , k - 1 as int) . add (list @ . subrange (k as int , list . len () as int))) ; new_list }

} // verus!