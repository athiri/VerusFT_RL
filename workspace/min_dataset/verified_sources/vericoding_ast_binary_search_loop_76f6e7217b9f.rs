use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn binary_search_precond (a : Seq < i32 > , key : i32) -> bool { forall | i : int , j : int | 0 <= i < j < a . len () ==> a [i] <= a [j] }
fn binary_search_loop (a : & Vec < i32 > , key : i32 , lo : usize , hi : usize) -> (result : usize) requires lo <= hi <= a . len () , binary_search_precond (a @ , key) , forall | i : int | 0 <= i < lo ==> a [i] < key , forall | i : int | hi <= i < a . len () ==> a [i] >= key , ensures result <= a . len () , forall | i : int | 0 <= i < result ==> a [i] < key , forall | i : int | result <= i < a . len () ==> a [i] >= key , decreases hi - lo { if lo == hi { return lo ; } let mid = lo + (hi - lo) / 2 ; if a [mid] < key { binary_search_loop (a , key , mid + 1 , hi) } else { binary_search_loop (a , key , lo , mid) } }

} // verus!