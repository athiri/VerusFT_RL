use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn shared_elements (list1 : & Vec < i32 > , list2 : & Vec < i32 >) -> (shared : Vec < i32 >) ensures forall | i : int | 0 <= i < shared . len () ==> (list1 @ . contains (# [trigger] shared [i]) && list2 @ . contains (# [trigger] shared [i] ,)) , forall | i : int , j : int | 0 <= i < j < shared . len () ==> shared [i] != shared [j] , { let mut result = Vec :: new () ; for i in 0 .. list1 . len () invariant forall | k : int | 0 <= k < result . len () ==> (list1 @ . contains (result [k]) && list2 @ . contains (result [k])) , forall | k1 : int , k2 : int | 0 <= k1 < k2 < result . len () ==> result [k1] != result [k2] , { let elem = list1 [i] ; let mut found_in_list2 = false ; for j in 0 .. list2 . len () invariant found_in_list2 == (exists | k : int | 0 <= k < j && list2 [k] == elem) , { if list2 [j] == elem { found_in_list2 = true ; } } if found_in_list2 { let mut already_added = false ; for k in 0 .. result . len () invariant already_added == (exists | m : int | 0 <= m < k && result [m] == elem) , { if result [k] == elem { already_added = true ; } } if ! already_added { result . push (elem) ; } } } result }

} // verus!