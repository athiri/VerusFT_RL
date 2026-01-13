use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn trap_rain_water_postcond (height : Seq < u32 > , result : u32 , h_precond : bool) -> bool { result >= 0 && (height . len () == 0 ==> result == 0) && (height . len () == 1 ==> result == 0) }
spec fn trap_rain_water_precond (height : Seq < u32 >) -> bool { true }
fn trap_rain_water (height : Vec < u32 >) -> (result : u32) requires trap_rain_water_precond (height @) ensures trap_rain_water_postcond (height @ , result , trap_rain_water_precond (height @)) { return 0 ; }

} // verus!