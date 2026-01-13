use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn main () { }
fn sub_array_at_index (main : & Vec < i32 > , sub : & Vec < i32 > , idx : usize) -> (result : bool) requires 0 <= idx <= (main . len () - sub . len ()) , ensures result == (main @ . subrange (idx as int , (idx + sub @ . len ())) =~= sub @) , { let mut i = 0 ; while i < sub . len () invariant 0 <= i <= sub . len () , idx + sub . len () <= main . len () , forall | j : int | 0 <= j < i ==> main @ [idx + j] == sub @ [j] , decreases sub . len () - i , { if main [idx + i] != sub [i] { assert (main @ [idx + i as int] != sub @ [i as int]) ; assert (main @ . subrange (idx as int , (idx + sub @ . len ())) =~= sub @ == false) ; return false ; } i += 1 ; } assert (forall | j : int | 0 <= j < sub . len () ==> main @ [idx + j] == sub @ [j]) ; assert (main @ . subrange (idx as int , (idx + sub @ . len ())) =~= sub @) ; true }

} // verus!