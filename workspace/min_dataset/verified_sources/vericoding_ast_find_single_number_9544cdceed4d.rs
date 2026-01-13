use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn find_single_number (nums : & Vec < i32 >) -> (result : i32) requires nums . len () > 0 , { return 0 ; }

} // verus!