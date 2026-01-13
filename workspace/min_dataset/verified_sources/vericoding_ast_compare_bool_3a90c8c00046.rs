use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn compare_bool (a : bool , b : bool) -> (eq : bool) requires true , ensures a == b ==> eq == true , a != b ==> eq == false , { a == b }

} // verus!