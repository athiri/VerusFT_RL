use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn add (x : i32 , y : i32) -> (res : Option < i32 >) ensures res . is_some () ==> res . unwrap () == x + y , { x . checked_add (y) }

} // verus!