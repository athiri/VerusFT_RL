use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn is_odd_at_index_odd_precond (a : Seq < i32 >) -> bool { true }
fn is_odd_at_index_odd (a : & Vec < i32 >) -> (result : bool) requires is_odd_at_index_odd_precond (a @) { return false ; }

} // verus!