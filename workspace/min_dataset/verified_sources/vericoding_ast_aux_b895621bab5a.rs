use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn aux (l : & Vec < i32 > , start : usize , increasing : bool , started_decreasing : bool) -> (res : bool) requires start < l . len () decreases l . len () - start { if start == l . len () - 1 { return started_decreasing ; } let current = l [start] ; let next = l [start + 1] ; if increasing { if current < next { aux (l , start + 1 , true , started_decreasing) } else if current > next { aux (l , start + 1 , false , true) } else { false } } else { if current > next { aux (l , start + 1 , false , started_decreasing) } else { false } } }

} // verus!