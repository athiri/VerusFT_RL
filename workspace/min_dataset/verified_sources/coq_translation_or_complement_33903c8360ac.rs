use vstd::prelude::*;

verus! {

pub open spec fn bool_not(a: bool) -> bool { !a }

pub open spec fn bool_or(a: bool, b: bool) -> bool { a || b }


pub proof fn or_complement(a: bool)
    ensures bool_or(a, bool_not(a)) == true
{}

} // verus!