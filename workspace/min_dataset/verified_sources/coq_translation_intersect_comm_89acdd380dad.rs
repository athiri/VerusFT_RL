use vstd::prelude::*;

verus! {

pub open spec fn set_intersect(s1: Set<nat>, s2: Set<nat>) -> Set<nat> {
    s1.intersect(s2)
}


pub proof fn intersect_comm(s1: Set<nat>, s2: Set<nat>)
    ensures set_intersect(s1, s2) == set_intersect(s2, s1)
{
}

} // verus!