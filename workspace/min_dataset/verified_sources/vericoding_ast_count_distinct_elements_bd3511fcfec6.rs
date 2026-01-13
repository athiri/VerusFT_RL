use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
# [verifier :: exec_allows_no_decreases_clause] fn count_distinct_elements (nums : & Vec < u32 > , start : usize , end : usize) -> (count : usize) requires start <= end , end < nums . len () , { return 0 ; }

} // verus!