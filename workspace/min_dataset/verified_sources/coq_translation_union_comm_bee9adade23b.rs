use vstd::prelude::*;

verus! {

pub open spec fn set_union(s1: Set<nat>, s2: Set<nat>) -> Set<nat> {
    s1.union(s2)
}


pub proof fn union_comm(s1: Set<nat>, s2: Set<nat>)
    ensures set_union(s1, s2) == set_union(s2, s1)
{
}

} // verus!