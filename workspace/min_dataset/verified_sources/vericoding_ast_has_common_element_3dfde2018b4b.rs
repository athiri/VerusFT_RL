use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn has_common_element_postcond (a : Seq < i32 > , b : Seq < i32 > , result : bool) -> bool { (exists | i : int , j : int | 0 <= i < a . len () && 0 <= j < b . len () && a [i] == b [j]) <==> result }
spec fn has_common_element_precond (a : Seq < i32 > , b : Seq < i32 >) -> bool { a . len () > 0 && b . len () > 0 }
fn has_common_element (a : & Vec < i32 > , b : & Vec < i32 >) -> (result : bool) requires has_common_element_precond (a @ , b @) , ensures has_common_element_postcond (a @ , b @ , result) , { let mut i = 0 ; while i < a . len () invariant 0 <= i <= a . len () , forall | ii : int , j : int | 0 <= ii < i && 0 <= j < b @ . len () ==> a @ [ii] != b @ [j] , decreases a . len () - i { let mut j = 0 ; while j < b . len () invariant 0 <= i < a . len () , 0 <= j <= b . len () , forall | jj : int | 0 <= jj < j ==> a @ [i as int] != b @ [jj] , forall | ii : int , jj : int | 0 <= ii < i && 0 <= jj < b @ . len () ==> a @ [ii] != b @ [jj] , decreases b . len () - j { if a [i] == b [j] { return true ; } j += 1 ; } i += 1 ; } false }

} // verus!