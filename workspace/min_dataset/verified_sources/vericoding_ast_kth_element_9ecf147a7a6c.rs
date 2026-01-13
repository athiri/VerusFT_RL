use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn kth_element_postcond (arr : Seq < i32 > , k : nat , result : int) -> bool { exists | i : int | 0 <= i < arr . len () && # [trigger] arr [i] == result && result == arr [k as int - 1] as int }
spec fn kth_element_precond (arr : Seq < i32 > , k : nat) -> bool { k >= 1 && k <= arr . len () }
fn kth_element (arr : & Vec < i32 > , k : usize) -> (result : i32) requires kth_element_precond (arr @ , k as nat) , ensures kth_element_postcond (arr @ , k as nat , result as int) , { let result = arr [k - 1] ; proof { let witness_idx = k as int - 1 ; assert (0 <= witness_idx < arr @ . len ()) by { assert (k >= 1 && k <= arr @ . len ()) ; assert (k as int >= 1) ; assert (k as int - 1 >= 0) ; assert (k as int <= arr @ . len ()) ; assert (k as int - 1 < arr @ . len ()) ; } ; assert (arr @ [witness_idx] == result) ; assert (result as int == arr @ [k as int - 1] as int) ; } result }

} // verus!