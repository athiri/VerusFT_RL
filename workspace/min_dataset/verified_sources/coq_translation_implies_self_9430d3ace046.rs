use vstd::prelude::*;

verus! {

pub open spec fn bool_implies(a: bool, b: bool) -> bool { !a || b }


pub proof fn implies_self(a: bool)
    ensures bool_implies(a, a) == true
{}

} // verus!