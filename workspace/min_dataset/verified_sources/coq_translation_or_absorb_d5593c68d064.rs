use vstd::prelude::*;

verus! {

pub open spec fn bool_and(a: bool, b: bool) -> bool { a && b }

pub open spec fn bool_or(a: bool, b: bool) -> bool { a || b }


pub proof fn or_absorb(a: bool, b: bool)
    ensures bool_or(a, bool_and(a, b)) == a
{}

} // verus!