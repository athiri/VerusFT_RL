use vstd::prelude::*;

verus! {

pub open spec fn bool_not(a: bool) -> bool { !a }

pub open spec fn bool_implies(a: bool, b: bool) -> bool { !a || b }


pub proof fn contrapositive(a: bool, b: bool)
    ensures bool_implies(a, b) == bool_implies(bool_not(b), bool_not(a))
{}

} // verus!