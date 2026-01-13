use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn reverse_to_k (list : & Vec < i32 > , n : usize) -> (reversed_list : Vec < i32 >) requires list @ . len () > 0 , 0 < n < list @ . len () , ensures reversed_list @ == list @ . subrange (0 , n as int) . reverse () . add (list @ . subrange (n as int , list . len () as int) ,) , { let mut result = Vec :: new () ; let mut i = n ; while i > 0 invariant i <= n , n < list @ . len () , result @ . len () == (n - i) as int , forall | j : int | 0 <= j < result @ . len () ==> # [trigger] result @ [j] == list @ [n as int - 1 - j] && { let idx = n as int - 1 - j ; 0 <= idx < list @ . len () } , decreases i , { i = i - 1 ; assert (i < n) ; assert (i < list . len ()) ; result . push (list [i]) ; } let mut j = n ; while j < list . len () invariant n <= j <= list . len () , n < list @ . len () , result @ . len () == (n + (j - n)) as int , forall | k : int | 0 <= k < n as int ==> # [trigger] result @ [k] == list @ [n as int - 1 - k] && { let idx = n as int - 1 - k ; 0 <= idx < list @ . len () } , forall | k : int | n as int <= k < result @ . len () ==> # [trigger] result @ [k] == list @ [k] , decreases list . len () - j , { assert (j < list . len ()) ; result . push (list [j]) ; j = j + 1 ; } result }

} // verus!