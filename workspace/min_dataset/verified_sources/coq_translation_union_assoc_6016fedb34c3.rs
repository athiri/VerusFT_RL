use vstd::prelude::*;

verus! {

pub open spec fn set_union(s1: Set<nat>, s2: Set<nat>) -> Set<nat> {
    s1.union(s2)
}


pub proof fn union_assoc(s1: Set<nat>, s2: Set<nat>, s3: Set<nat>)
    ensures set_union(set_union(s1, s2), s3) == set_union(s1, set_union(s2, s3))
{
    assert(set_union(set_union(s1, s2), s3) =~= set_union(s1, set_union(s2, s3)));
}

} // verus!