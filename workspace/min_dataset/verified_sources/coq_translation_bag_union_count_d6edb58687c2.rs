use vstd::prelude::*;

verus! {

pub struct Bag {
    pub counts: Map<nat, nat>,
}

pub open spec fn bag_union(b1: Bag, b2: Bag) -> Bag {
    Bag {
        counts: Map::new(
            |k: nat| b1.counts.dom().contains(k) || b2.counts.dom().contains(k),
            |k: nat| bag_count(b1, k) + bag_count(b2, k),
        ),
    }
}

pub open spec fn bag_count(b: Bag, x: nat) -> nat {
    if b.counts.dom().contains(x) {
        b.counts[x]
    } else {
        0
    }
}


pub proof fn bag_union_count(b1: Bag, b2: Bag, x: nat)
    ensures bag_count(bag_union(b1, b2), x) == bag_count(b1, x) + bag_count(b2, x)
{
}

} // verus!