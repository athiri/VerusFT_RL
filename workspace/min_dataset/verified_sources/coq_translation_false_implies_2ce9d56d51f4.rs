use vstd::prelude::*;

verus! {

pub open spec fn bool_implies(a: bool, b: bool) -> bool { !a || b }


pub proof fn false_implies(b: bool)
    ensures bool_implies(false, b) == true
{}

} // verus!