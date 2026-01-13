use vstd::prelude::*;

verus! {

pub proof fn bag_union_count(b1: Bag, b2: Bag, x: nat)
    ensures bag_count(bag_union(b1, b2), x) == bag_count(b1, x) + bag_count(b2, x)
{
}


pub struct Bag {
    pub counts: Map<nat, nat>,
}

pub open spec fn bag_count(b: Bag, x: nat) -> nat {
    if b.counts.dom().contains(x) {
        b.counts[x]
    } else {
        0
    }
}

pub open spec fn bag_union(b1: Bag, b2: Bag) -> Bag {
    Bag {
        counts: Map::new(
            |k: nat| b1.counts.dom().contains(k) || b2.counts.dom().contains(k),
            |k: nat| bag_count(b1, k) + bag_count(b2, k),
        ),
    }
}

pub open spec fn bag_eq(b1: Bag, b2: Bag) -> bool {
    forall|x: nat| bag_count(b1, x) == bag_count(b2, x)
}


pub proof fn bag_union_comm(b1: Bag, b2: Bag)
    ensures bag_eq(bag_union(b1, b2), bag_union(b2, b1))
{
    assert forall|x: nat| bag_count(bag_union(b1, b2), x) == bag_count(bag_union(b2, b1), x) by {
        bag_union_count(b1, b2, x);
        bag_union_count(b2, b1, x);
    }
}

} // verus!