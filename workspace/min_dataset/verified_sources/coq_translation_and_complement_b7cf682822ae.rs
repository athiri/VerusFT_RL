use vstd::prelude::*;

verus! {

pub open spec fn bool_not(a: bool) -> bool { !a }

pub open spec fn bool_and(a: bool, b: bool) -> bool { a && b }


pub proof fn and_complement(a: bool)
    ensures bool_and(a, bool_not(a)) == false
{}

} // verus!