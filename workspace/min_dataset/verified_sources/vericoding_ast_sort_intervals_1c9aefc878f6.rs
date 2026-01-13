use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn sort_intervals (intervals : & mut Vec < (usize , usize) >) ensures intervals @ . len () == old (intervals) @ . len () { }

} // verus!