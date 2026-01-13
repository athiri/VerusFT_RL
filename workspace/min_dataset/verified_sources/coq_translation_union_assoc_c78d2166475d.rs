use vstd::prelude::*;

verus! {

pub proof fn count_union(m1: Multiset, m2: Multiset, x: nat)
    ensures count(union(m1, m2), x) == count(m1, x) + count(m2, x)
{
}


pub open spec fn count(m: Multiset, x: nat) -> nat {
    if m.counts.dom().contains(x) {
        m.counts[x]
    } else {
        0
    }
}


pub struct Multiset {
    pub counts: Map<nat, nat>,
}

pub open spec fn multiset_eq(m1: Multiset, m2: Multiset) -> bool {
    forall|x: nat| count(m1, x) == count(m2, x)
}

pub open spec fn union(m1: Multiset, m2: Multiset) -> Multiset {
    Multiset {
        counts: Map::new(
            |k: nat| m1.counts.dom().contains(k) || m2.counts.dom().contains(k),
            |k: nat| count(m1, k) + count(m2, k),
        ),
    }
}


pub proof fn union_assoc(m1: Multiset, m2: Multiset, m3: Multiset)
    ensures multiset_eq(union(union(m1, m2), m3), union(m1, union(m2, m3)))
{
    assert forall|x: nat| count(union(union(m1, m2), m3), x) ==
        count(union(m1, union(m2, m3)), x) by {
        count_union(m1, m2, x);
        count_union(m2, m3, x);
        count_union(union(m1, m2), m3, x);
        count_union(m1, union(m2, m3), x);
    }
}

} // verus!