use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn bubbleSorta (a : & mut Vec < int > , c : usize , f : usize) requires 0 <= c <= f <= old (a) . len () , ensures a . len () == old (a) . len () , { }

} // verus!