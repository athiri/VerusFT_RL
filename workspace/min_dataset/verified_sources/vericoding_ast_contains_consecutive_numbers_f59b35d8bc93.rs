use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn contains_consecutive_numbers (arr : & Vec < i32 >) -> (is_consecutive : bool) requires arr . len () > 0 , forall | i : int | 0 <= i < arr . len () ==> (0 <= # [trigger] arr [i] + 1 < MAX) , ensures is_consecutive == (forall | i : int , j : int | 0 <= i < j < arr . len () && j == i + 1 ==> (arr [i] + 1 == arr [j])) , { for idx in 0 .. arr . len () - 1 invariant arr . len () > 0 , forall | i : int | 0 <= i < arr . len () ==> (0 <= # [trigger] arr [i] + 1 < MAX) , forall | i : int , j : int | 0 <= i < j < arr . len () && j == i + 1 && j <= idx ==> (arr [i] + 1 == arr [j]) , { assert (idx < arr . len () - 1) ; assert (idx + 1 < arr . len ()) ; assert (0 <= arr [idx as int] + 1 < MAX) ; if arr [idx] + 1 != arr [idx + 1] { return false ; } } return true ; }

} // verus!