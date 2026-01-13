use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn find_all_occurrences (text : Seq < char > , pattern : Seq < char >) -> (result : Vec < usize >) requires true ensures true { return Vec :: new () ; }

} // verus!