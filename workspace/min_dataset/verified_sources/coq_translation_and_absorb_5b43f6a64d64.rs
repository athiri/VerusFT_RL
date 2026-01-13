use vstd::prelude::*;

verus! {

pub open spec fn bool_and(a: bool, b: bool) -> bool { a && b }

pub open spec fn bool_or(a: bool, b: bool) -> bool { a || b }


pub proof fn and_absorb(a: bool, b: bool)
    ensures bool_and(a, bool_or(a, b)) == a
{}

} // verus!