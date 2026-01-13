use vstd::prelude::*;

verus! {

pub open spec fn bool_and(a: bool, b: bool) -> bool { a && b }


pub proof fn and_true(a: bool) ensures bool_and(a, true) == a {}

} // verus!