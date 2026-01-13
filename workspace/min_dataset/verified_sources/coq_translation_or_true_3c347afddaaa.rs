use vstd::prelude::*;

verus! {

pub open spec fn bool_or(a: bool, b: bool) -> bool { a || b }


pub proof fn or_true(a: bool) ensures bool_or(a, true) == true {}

} // verus!