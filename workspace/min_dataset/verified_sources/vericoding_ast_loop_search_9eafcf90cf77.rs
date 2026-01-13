use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn loop_search (nums : & Vec < i32 > , start : usize , first : i32 , second : i32) -> (result : bool) requires start <= nums . len () decreases nums . len () - start { return false ; }

} // verus!