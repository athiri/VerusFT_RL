use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn main () { }
fn sub_array_at_index (main : & Vec < i32 > , sub : & Vec < i32 > , idx : usize) -> (result : bool) requires 0 <= idx <= (main . len () - sub . len ()) , ensures result == (main @ . subrange (idx as int , (idx + sub @ . len ())) =~= sub @) , { let mut i = 0 ; while i < sub . len () invariant 0 <= i <= sub . len () , idx + sub . len () <= main . len () , forall | j : int | 0 <= j < i ==> main @ [idx as int + j] == sub @ [j] , decreases sub . len () - i , { assert (idx + i < main . len ()) ; if main [idx + i] != sub [i] { return false ; } i += 1 ; } assert (forall | j : int | 0 <= j < sub . len () ==> main @ [idx as int + j] == sub @ [j]) ; true }

} // verus!