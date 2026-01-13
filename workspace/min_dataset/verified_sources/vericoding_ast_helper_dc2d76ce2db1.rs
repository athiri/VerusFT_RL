use vstd::prelude::*;
use vstd :: arithmetic :: power :: pow ;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn helper (n : u32) -> (result : bool) decreases n { return false ; }

} // verus!