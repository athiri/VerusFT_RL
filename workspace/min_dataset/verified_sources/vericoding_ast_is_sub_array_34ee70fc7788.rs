use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn is_subrange_at (main : Seq < i32 > , sub : Seq < i32 > , i : int) -> bool { sub =~= main . subrange (i , i + sub . len ()) }
fn main () { }
fn sub_array_at_index (main : & Vec < i32 > , sub : & Vec < i32 > , idx : usize) -> (result : bool) requires 0 <= idx <= (main . len () - sub . len ()) , ensures result == (main @ . subrange (idx as int , (idx + sub @ . len ())) =~= sub @) , { let mut i = 0 ; while i < sub . len () invariant 0 <= i <= sub . len () , idx + sub . len () <= main . len () , forall | j : int | 0 <= j < i ==> main @ [idx + j] == sub @ [j] , decreases sub . len () - i , { if main [idx + i] != sub [i] { assert (main @ [idx + i as int] != sub @ [i as int]) ; assert (main @ . subrange (idx as int , (idx + sub @ . len ())) =~= sub @ == false) ; return false ; } i += 1 ; } assert (forall | j : int | 0 <= j < sub . len () ==> main @ [idx + j] == sub @ [j]) ; assert (main @ . subrange (idx as int , (idx + sub @ . len ())) =~= sub @) ; true }
fn is_sub_array (main : & Vec < i32 > , sub : & Vec < i32 >) -> (result : bool) ensures result == (exists | k : int | 0 <= k <= (main . len () - sub . len ()) && is_subrange_at (main @ , sub @ , k)) , { if sub . len () > main . len () { assert (forall | k : int | 0 <= k <= (main . len () - sub . len ()) ==> ! is_subrange_at (main @ , sub @ , k)) ; return false ; } let mut idx = 0 ; while idx <= main . len () - sub . len () invariant 0 <= idx <= main . len () - sub . len () + 1 , sub . len () <= main . len () , forall | k : int | 0 <= k < idx ==> ! is_subrange_at (main @ , sub @ , k) , decreases main . len () - sub . len () + 1 - idx , { assert (idx <= main . len () - sub . len ()) ; if sub_array_at_index (main , sub , idx) { assert (is_subrange_at (main @ , sub @ , idx as int)) ; return true ; } idx += 1 ; } assert (forall | k : int | 0 <= k <= (main . len () - sub . len ()) ==> ! is_subrange_at (main @ , sub @ , k)) ; false }

} // verus!