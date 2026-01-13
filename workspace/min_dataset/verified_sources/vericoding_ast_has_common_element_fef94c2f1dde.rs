use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn has_common_element (list1 : & Vec < i32 > , list2 : & Vec < i32 >) -> (result : bool) ensures result == (exists | i : int , j : int | 0 <= i < list1 . len () && 0 <= j < list2 . len () && (list1 [i] == list2 [j])) , { let mut i : usize = 0 ; while i < list1 . len () invariant forall | ii : int , jj : int | 0 <= ii < i && 0 <= jj < list2 . len () ==> list1 [ii] != list2 [jj] , decreases list1 . len () - i { let mut j : usize = 0 ; while j < list2 . len () invariant i < list1 . len () , forall | ii : int , jj : int | 0 <= ii < i && 0 <= jj < list2 . len () ==> list1 [ii] != list2 [jj] , forall | jj : int | 0 <= jj < j ==> list1 [i as int] != list2 [jj] , decreases list2 . len () - j { if list1 [i] == list2 [j] { return true ; } j += 1 ; } i += 1 ; } false }

} // verus!