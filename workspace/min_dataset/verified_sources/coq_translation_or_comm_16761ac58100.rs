use vstd::prelude::*;

verus! {

pub open spec fn bool_or(a: bool, b: bool) -> bool { a || b }


pub proof fn or_comm(a: bool, b: bool) ensures bool_or(a, b) == bool_or(b, a) {}

} // verus!