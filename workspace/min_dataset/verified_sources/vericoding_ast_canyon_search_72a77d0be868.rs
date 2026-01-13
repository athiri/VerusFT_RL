use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn canyon_search (a : & [i32] , b : & [i32]) -> (d : u32) requires a . len () != 0 && b . len () != 0 , forall | i : int , j : int | 0 <= i < j < a . len () ==> a [i] <= a [j] , forall | i : int , j : int | 0 <= i < j < b . len () ==> b [i] <= b [j] , ensures true , { return 0 ; }

} // verus!