use vstd::prelude::*;

verus! {

pub open spec fn bool_or(a: bool, b: bool) -> bool { a || b }

pub open spec fn bool_not(a: bool) -> bool { !a }

pub open spec fn bool_implies(a: bool, b: bool) -> bool { !a || b }


pub proof fn implies_def(a: bool, b: bool)
    ensures bool_implies(a, b) == bool_or(bool_not(a), b)
{}

} // verus!