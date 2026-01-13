use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn reverse_to_k (list : & Vec < i32 > , n : usize) -> (reversed_list : Vec < i32 >) requires list @ . len () > 0 , 0 < n < list @ . len () , ensures reversed_list @ == list @ . subrange (0 , n as int) . reverse () . add (list @ . subrange (n as int , list . len () as int) ,) , { let mut result = Vec :: new () ; let mut i = n ; while i > 0 invariant i <= n , n < list @ . len () , result @ . len () == (n - i) as int , result @ == list @ . subrange (i as int , n as int) . reverse () , decreases i , { i = i - 1 ; assert (i < n) ; assert (n < list @ . len ()) ; assert (i < list @ . len ()) ; result . push (list [i]) ; } let mut j = n ; while j < list . len () invariant n <= j <= list . len () , result @ . len () == (n + (j - n)) as int , result @ == list @ . subrange (0 , n as int) . reverse () . add (list @ . subrange (n as int , j as int)) , decreases list . len () - j , { result . push (list [j]) ; j = j + 1 ; } result }

} // verus!