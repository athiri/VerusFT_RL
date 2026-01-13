use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn triple_postcond (x : int , result : int) -> bool { result / 3 == x && result / 3 * 3 == result }
spec fn triple_precond (x : int) -> bool { true }
fn triple (x : u32) -> (result : u32) requires triple_precond (x as int) , x <= MAX / 3 ensures triple_postcond (x as int , result as int) { 3 * x }

} // verus!