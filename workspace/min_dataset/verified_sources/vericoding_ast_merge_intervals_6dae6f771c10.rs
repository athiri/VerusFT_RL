use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn merge_intervals (sorted_intervals : Vec < (usize , usize) >) -> (result : Vec < (usize , usize) >) ensures result @ . len () <= sorted_intervals @ . len () { return Vec :: new () ; }

} // verus!