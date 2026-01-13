use vstd::prelude::*;

verus! {

pub open spec fn bool_and(a: bool, b: bool) -> bool { a && b }


pub proof fn and_comm(a: bool, b: bool) ensures bool_and(a, b) == bool_and(b, a) {}

} // verus!