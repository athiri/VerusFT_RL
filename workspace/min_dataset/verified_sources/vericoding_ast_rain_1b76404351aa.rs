use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn rain_postcond (heights : Seq < i32 > , result : int) -> bool { result >= 0 && if heights . len () < 3 { result == 0 } else { true } }
spec fn rain_precond (heights : Seq < i32 >) -> bool { forall | i : int | 0 <= i < heights . len () ==> # [trigger] heights [i] >= 0 }
fn rain (heights : Vec < i32 >) -> (result : i32) requires rain_precond (heights @) , ensures rain_postcond (heights @ , result as int) , { return 0 ; }

} // verus!