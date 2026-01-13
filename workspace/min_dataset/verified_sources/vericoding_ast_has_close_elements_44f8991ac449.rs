use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn has_close_elements_precond (numbers : Seq < i32 > , threshold : i32) -> bool { threshold >= 0 && forall | i : int | 0 <= i < numbers . len () ==> numbers [i] >= - 1000000 && numbers [i] <= 1000000 }
fn has_close_elements (numbers : Vec < i32 > , threshold : i32) -> (result : bool) requires has_close_elements_precond (numbers @ , threshold) , { return false ; }

} // verus!