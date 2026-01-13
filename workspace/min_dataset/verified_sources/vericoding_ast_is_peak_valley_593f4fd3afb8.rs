use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn is_peak_valley_precond (lst : Seq < i32 >) -> bool { true }
spec fn is_peak_valley_postcond (lst : Seq < i32 > , result : bool) -> bool { (lst . len () <= 2 ==> ! result) }
fn aux (l : & Vec < i32 > , start : usize , increasing : bool , started_decreasing : bool) -> (res : bool) requires start < l . len () decreases l . len () - start { if start == l . len () - 1 { return started_decreasing ; } let current = l [start] ; let next = l [start + 1] ; if increasing { if current < next { aux (l , start + 1 , true , false) } else if current > next { if start == 0 { false } else { aux (l , start + 1 , false , true) } } else { false } } else { if current > next { aux (l , start + 1 , false , true) } else { false } } }
fn is_peak_valley (lst : Vec < i32 >) -> (result : bool) requires is_peak_valley_precond (lst @) ensures is_peak_valley_postcond (lst @ , result) { if lst . len () <= 2 { return false ; } aux (& lst , 0 , true , false) }

} // verus!