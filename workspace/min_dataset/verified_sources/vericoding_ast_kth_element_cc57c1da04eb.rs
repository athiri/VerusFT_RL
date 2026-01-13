use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn kth_element_precond (arr : Seq < i32 > , k : nat) -> bool { k >= 1 && k <= arr . len () }
spec fn kth_element_postcond (arr : Seq < i32 > , k : nat , result : int) -> bool { exists | i : int | 0 <= i < arr . len () && # [trigger] arr [i] == result && result == arr [k as int - 1] as int }
fn kth_element (arr : & Vec < i32 > , k : usize) -> (result : i32) requires kth_element_precond (arr @ , k as nat) , ensures kth_element_postcond (arr @ , k as nat , result as int) , { arr [k - 1] }

} // verus!