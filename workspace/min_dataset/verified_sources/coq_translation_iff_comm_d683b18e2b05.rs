use vstd::prelude::*;

verus! {

pub open spec fn bool_iff(a: bool, b: bool) -> bool { a == b }


pub proof fn iff_comm(a: bool, b: bool) ensures bool_iff(a, b) == bool_iff(b, a) {}

} // verus!