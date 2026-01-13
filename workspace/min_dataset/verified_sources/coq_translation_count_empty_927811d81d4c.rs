use vstd::prelude::*;

verus! {

pub struct Multiset {
    pub counts: Map<nat, nat>,
}

pub open spec fn empty() -> Multiset {
    Multiset { counts: Map::empty() }
}

pub open spec fn count(m: Multiset, x: nat) -> nat {
    if m.counts.dom().contains(x) {
        m.counts[x]
    } else {
        0
    }
}


pub proof fn count_empty(x: nat)
    ensures count(empty(), x) == 0
{
}

} // verus!