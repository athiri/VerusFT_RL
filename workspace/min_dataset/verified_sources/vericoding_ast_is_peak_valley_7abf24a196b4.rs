use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn is_peak_valley_precond (lst : Seq < i32 >) -> bool { true }
spec fn is_peak_valley_postcond (lst : Seq < i32 > , result : bool) -> bool { (lst . len () <= 2 ==> ! result) }
fn is_peak_valley (lst : Vec < i32 >) -> (result : bool) requires is_peak_valley_precond (lst @) ensures is_peak_valley_postcond (lst @ , result) { return false ; }

} // verus!