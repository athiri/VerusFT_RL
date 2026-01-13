use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn isEven_precond (n : int) -> bool { true }
spec fn isEven_postcond (n : int , result : bool) -> bool { (result ==> n % 2 == 0) && (! result ==> n % 2 != 0) }
fn isEven (n : i64) -> (result : bool) requires isEven_precond (n as int) ensures isEven_postcond (n as int , result) { n % 2 == 0 }

} // verus!