use vstd::prelude::*;

verus! {

pub proof fn count_union(m1: Multiset, m2: Multiset, x: nat)
    ensures count(union(m1, m2), x) == count(m1, x) + count(m2, x)
{
}


pub open spec fn union(m1: Multiset, m2: Multiset) -> Multiset {
    Multiset {
        counts: Map::new(
            |k: nat| m1.counts.dom().contains(k) || m2.counts.dom().contains(k),
            |k: nat| count(m1, k) + count(m2, k),
        ),
    }
}

pub open spec fn multiset_eq(m1: Multiset, m2: Multiset) -> bool {
    forall|x: nat| count(m1, x) == count(m2, x)
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


pub struct Multiset {
    pub counts: Map<nat, nat>,
}

pub open spec fn empty() -> Multiset {
    Multiset { counts: Map::empty() }
}


pub proof fn union_empty_left(m: Multiset)
    ensures multiset_eq(union(empty(), m), m)
{
    assert forall|x: nat| count(union(empty(), m), x) == count(m, x) by {
        count_empty(x);
        count_union(empty(), m, x);
    }
}

} // verus!