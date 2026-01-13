use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn binary_search (a : & Vec < i32 > , circle : i32) -> (n : usize) requires true , ensures 0 <= n <= a . len () , { 0 }

} // verus!