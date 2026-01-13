use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn binary_search_position (dp : & Vec < i32 > , x : i32) -> (pos : usize) ensures pos <= dp . len () { return 0 ; }

} // verus!