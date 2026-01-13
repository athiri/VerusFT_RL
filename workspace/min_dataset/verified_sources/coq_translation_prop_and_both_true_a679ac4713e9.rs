use vstd::prelude::*;

verus! {

pub open spec fn prop_and(p: bool, q: bool) -> bool {
    p && q
}


pub proof fn prop_and_both_true(p: bool, q: bool)
    requires p, q
    ensures prop_and(p, q)
{
}

} // verus!