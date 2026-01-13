use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn move_zeros_to_end_postcond (arr : Seq < i32 > , result : Seq < i32 >) -> bool { let first_zero_idx = first_zero_index (result) ; &&& is_perm (result , arr) &&& result . subrange (0 , first_zero_idx) == filter_non_zeros (arr) &&& result . subrange (first_zero_idx , result . len () as int) == filter_zeros (arr) }
spec fn move_zeros_to_end_precond (arr : Seq < i32 >) -> bool { true }
spec fn filter_zeros (s : Seq < i32 >) -> Seq < i32 > { s . filter (| x : i32 | x == 0) }
spec fn first_zero_index (s : Seq < i32 >) -> int { 0 }
spec fn filter_non_zeros (s : Seq < i32 >) -> Seq < i32 > { s . filter (| x : i32 | x != 0) }
spec fn is_perm < T > (s1 : Seq < T > , s2 : Seq < T >) -> bool { s1 . to_multiset () == s2 . to_multiset () }
# [verifier :: external_body] fn move_zeros_to_end (arr : Vec < i32 >) -> (result : Vec < i32 >) requires move_zeros_to_end_precond (arr @) , ensures move_zeros_to_end_postcond (arr @ , result @) , { return Vec :: new () ; }

} // verus!