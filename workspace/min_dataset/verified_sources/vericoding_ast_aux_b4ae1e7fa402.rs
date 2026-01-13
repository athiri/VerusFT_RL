use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn aux (l : & Vec < i32 > , start : usize , increasing : bool , started_decreasing : bool) -> (res : bool) requires start < l . len () decreases l . len () - start { return false ; }

} // verus!