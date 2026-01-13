use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn unique_sorted_precond (arr : Seq < int >) -> bool { true }
spec fn unique_sorted_postcond (arr : Seq < int > , result : Seq < int >) -> bool { true }
fn unique_sorted (arr : Vec < int >) -> (result : Vec < int >) requires unique_sorted_precond (arr @) ensures unique_sorted_postcond (arr @ , result @) { return Vec :: new () ; }

} // verus!