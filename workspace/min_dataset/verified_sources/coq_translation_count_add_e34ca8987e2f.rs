use vstd::prelude::*;

verus! {

pub struct Multiset {
    pub counts: Map<nat, nat>,
}

pub open spec fn count(m: Multiset, x: nat) -> nat {
    if m.counts.contains_key(x) {
        m.counts[x]
    } else {
        0
    }
}

pub open spec fn add(m: Multiset, x: nat) -> Multiset {
    Multiset { counts: m.counts.insert(x, count(m, x) + 1) }
}

pub proof fn count_add(m: Multiset, x: nat, y: nat)
    ensures count(add(m, x), y) == if x == y { count(m, y) + 1 } else { count(m, y) }
{
}

} // verus!
