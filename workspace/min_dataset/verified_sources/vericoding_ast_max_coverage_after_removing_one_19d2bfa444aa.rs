use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn max_coverage_after_removing_one (intervals : Vec < (usize , usize) >) -> (result : usize) requires intervals @ . len () > 0 , forall | i : int | 0 <= i < intervals @ . len () ==> # [trigger] intervals @ [i] . 0 <= intervals @ [i] . 1 ensures result >= 0 { return 0 ; }

} // verus!