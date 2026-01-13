use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
pub fn generate_all_combinations (n : i32 , k : i32) -> Vec < Vec < i32 > > requires 0 <= n <= MAX - 1 , 0 <= k <= n , { return Vec :: new () ; }

} // verus!