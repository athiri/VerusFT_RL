use vstd::prelude::*;

verus! {

pub open spec fn bool_or(a: bool, b: bool) -> bool { a || b }


pub proof fn or_false(a: bool) ensures bool_or(a, false) == a {}

} // verus!