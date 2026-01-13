use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn my_min_precond (a : int , b : int) -> bool { true }
spec fn my_min_postcond (a : int , b : int , result : int) -> bool { (result <= a && result <= b) && (result == a || result == b) }
fn my_min (a : i32 , b : i32) -> (result : i32) requires my_min_precond (a as int , b as int) , ensures my_min_postcond (a as int , b as int , result as int) , { if a <= b { a } else { b } }

} // verus!