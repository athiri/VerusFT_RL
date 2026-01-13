use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn F_spec () -> int { 0 }
fn F () -> (r : i32) ensures r == F_spec () && r <= 0 { return 0 ; }

} // verus!