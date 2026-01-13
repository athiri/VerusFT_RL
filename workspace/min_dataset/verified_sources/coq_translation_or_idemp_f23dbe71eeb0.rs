use vstd::prelude::*;

verus! {

pub open spec fn bool_or(a: bool, b: bool) -> bool { a || b }


pub proof fn or_idemp(a: bool) ensures bool_or(a, a) == a {}

} // verus!