use vstd::prelude::*;

verus! {

pub open spec fn set_intersect(s1: Set<nat>, s2: Set<nat>) -> Set<nat> {
    s1.intersect(s2)
}

pub open spec fn set_empty() -> Set<nat> {
    Set::empty()
}


pub proof fn intersect_empty(s: Set<nat>)
    ensures set_intersect(s, set_empty()) == set_empty()
{
    assert(set_intersect(s, set_empty()) =~= set_empty());
}

} // verus!