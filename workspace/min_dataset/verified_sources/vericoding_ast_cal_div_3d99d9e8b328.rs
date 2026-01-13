use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn cal_div () -> (r : (u32 , u32)) ensures r . 0 == 27 , r . 1 == 2 , { (27 , 2) }

} // verus!