use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn isSublist_precond (sub : Seq < i32 > , main : Seq < i32 >) -> bool { true }
fn main () { }
fn isSublist (sub : Vec < i32 > , main : Vec < i32 >) -> (result : bool) requires isSublist_precond (sub @ , main @) { return false ; }

} // verus!