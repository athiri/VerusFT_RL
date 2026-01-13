use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn is_sorted_postcond (a : Seq < i32 > , result : bool) -> bool { (forall | i : int | 0 <= i < a . len () - 1 ==> # [trigger] a [i] <= a [i + 1]) <==> result }
spec fn is_sorted_precond (a : Seq < i32 >) -> bool { true }
fn is_sorted (a : & [i32]) -> (result : bool) requires is_sorted_precond (a @) , ensures is_sorted_postcond (a @ , result) , { if a . len () <= 1 { return true ; } let mut i = 0 ; while i < a . len () - 1 invariant 0 <= i <= a . len () - 1 , forall | j : int | 0 <= j < i ==> # [trigger] a @ [j] <= a @ [j + 1] , decreases a . len () - i , { if a [i] > a [i + 1] { return false ; } i += 1 ; } true }

} // verus!