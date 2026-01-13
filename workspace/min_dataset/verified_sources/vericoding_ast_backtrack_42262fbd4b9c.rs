use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn backtrack (nums : & Vec < i32 > , current_permutation : & mut Vec < i32 > , used : & mut Vec < bool > , result : & mut Vec < Vec < i32 > > ,) requires nums . len () == old (used) . len () , ensures used . len () == old (used) . len () , { }

} // verus!