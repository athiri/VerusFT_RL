use vstd::prelude::*;

verus! {

pub open spec fn prop_or(p: bool, q: bool) -> bool {
    p || q
}


pub proof fn prop_or_one_true(p: bool, q: bool)
    requires p || q
    ensures prop_or(p, q)
{
}

} // verus!