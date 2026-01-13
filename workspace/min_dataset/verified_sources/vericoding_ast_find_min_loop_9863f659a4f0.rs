use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn find_min_loop (a : & Vec < i32 > , i : usize , current_min : i32) -> (result : i32) requires a . len () > 0 , i <= a . len () , exists | j : int | 0 <= j < a . len () && current_min == a [j as int] , forall | j : int | 0 <= j < i ==> current_min <= a [j as int] , ensures exists | k : int | 0 <= k < a . len () && result == a [k as int] , forall | j : int | 0 <= j < a . len () ==> result <= a [j as int] , decreases a . len () - i , { if i == a . len () { current_min } else { let new_min = if a [i] < current_min { a [i] } else { current_min } ; find_min_loop (a , i + 1 , new_min) } }

} // verus!