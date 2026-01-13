use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn max_subarray_sum_divisible_by_k_postcond (arr : Seq < i32 > , k : i32 , result : i32) -> bool { let result_int = result as int ; (result == 0 ==> (forall | start : int , len : int | # ! [auto] is_divisible_subarray (arr , start , len , k) ==> get_subarray_sum (arr , start , len) <= 0)) && (result != 0 ==> ((exists | start : int , len : int | # ! [auto] is_divisible_subarray (arr , start , len , k) && get_subarray_sum (arr , start , len) == result_int) && (forall | start : int , len : int | # ! [auto] is_divisible_subarray (arr , start , len , k) ==> get_subarray_sum (arr , start , len) <= result_int))) }
spec fn max_subarray_sum_divisible_by_k_precond (arr : Seq < i32 > , k : i32) -> bool { k > 0 }
spec fn get_subarray_sum (arr : Seq < i32 > , start : int , len : int) -> int { if 0 <= start && start + len <= arr . len () && len >= 0 { array_sum (arr . subrange (start , start + len)) } else { 0int } }
spec fn is_divisible_subarray (arr : Seq < i32 > , start : int , len : int , k : i32) -> bool { 0 <= start && start + len <= arr . len () && len > 0 && len % (k as int) == 0 }
spec fn array_sum (arr : Seq < i32 >) -> int decreases arr . len () { if arr . len () == 0 { 0int } else { arr [0] as int + array_sum (arr . subrange (1 , arr . len () as int)) } }
# [verifier :: external_body] fn max_subarray_sum_divisible_by_k (arr : & Vec < i32 > , k : i32) -> (result : i32) requires max_subarray_sum_divisible_by_k_precond (arr @ , k) , ensures max_subarray_sum_divisible_by_k_postcond (arr @ , k , result) , { let mut max_sum : i32 = 0 ; let n = arr . len () ; let mut start = 0 ; while start < n { let mut len = k as usize ; while start + len <= n { let mut current_sum : i32 = 0 ; let mut i = start ; while i < start + len { current_sum = current_sum + arr [i] ; i += 1 ; } if current_sum > max_sum { max_sum = current_sum ; } len += k as usize ; } start += 1 ; } max_sum }

} // verus!