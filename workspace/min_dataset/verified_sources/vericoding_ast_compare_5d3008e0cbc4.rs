use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn compare_postcond (a : int , b : int , result : bool) -> bool { (a == b ==> result == true) && (a != b ==> result == false) }
spec fn compare_precond (a : int , b : int) -> bool { true }
fn compare (a : int , b : int) -> (result : bool) requires compare_precond (a , b) ensures compare_postcond (a , b , result) { a == b }

} // verus!