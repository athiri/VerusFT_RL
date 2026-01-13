use vstd::prelude::*;

verus! {

pub open spec fn set_union(s1: Set<nat>, s2: Set<nat>) -> Set<nat> {
    s1.union(s2)
}

pub open spec fn set_empty() -> Set<nat> {
    Set::empty()
}


pub proof fn union_empty(s: Set<nat>)
    ensures set_union(s, set_empty()) == s
{
    assert(set_union(s, set_empty()) =~= s);
}

} // verus!