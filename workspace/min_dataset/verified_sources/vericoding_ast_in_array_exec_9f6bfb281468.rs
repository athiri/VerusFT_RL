use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn in_array (a : Seq < i32 > , x : i32) -> (ret : bool) { exists | i : int | 0 <= i < a . len () && a [i] == x }
fn in_array_exec (a : & Vec < i32 > , x : i32) -> (result : bool) ensures result == in_array (a @ , x) , { for i in 0 .. a . len () invariant forall | j : int | 0 <= j < i ==> a [j] != x , { if a [i] == x { return true ; } } return false ; }

} // verus!