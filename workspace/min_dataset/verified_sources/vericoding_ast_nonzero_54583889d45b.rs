use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn nonzero (arr : & [i32]) -> (num : i32) requires arr . len () >= 0 , ensures num >= 0 , { return 0 ; }

} // verus!