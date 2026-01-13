use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn separate (v : & mut Vec < i32 >) -> (result : usize) requires old (v) . len () > 0 { return 0 ; }

} // verus!