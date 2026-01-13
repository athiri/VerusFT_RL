use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn max_array_precond (a : & Vec < i32 >) -> bool { a . len () > 0 }
spec fn max_array_aux_spec (a : & Vec < i32 > , index : nat , current : i32) -> i32 recommends index <= a . len () , a . len () > 0 , decreases a . len () - index , { if index < a . len () { let new_current = if current > a [index as int] { current } else { a [index as int] } ; max_array_aux_spec (a , index + 1 , new_current) } else { current } }
fn max_array_aux (a : & Vec < i32 > , index : usize , current : i32) -> (result : i32) requires index <= a . len () , a . len () > 0 , ensures result == max_array_aux_spec (a , index as nat , current) , result >= current , forall | k : int | index <= k < a . len () ==> result >= a [k] , (exists | k : int | index <= k < a . len () && result == a [k]) || result == current , decreases a . len () - index , { if index < a . len () { let new_current = if current > a [index] { current } else { a [index] } ; max_array_aux (a , index + 1 , new_current) } else { current } }
fn max_array (a : & Vec < i32 >) -> (result : i32) requires max_array_precond (a) , ensures result == max_array_aux_spec (a , 1nat , a [0]) , forall | k : int | 0 <= k < a . len () ==> result >= a [k] , exists | k : int | 0 <= k < a . len () && result == a [k] , { max_array_aux (a , 1 , a [0]) }

} // verus!