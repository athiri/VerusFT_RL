use vstd::prelude::*;

verus! {

pub open spec fn bag_count(b: Bag, x: nat) -> nat {
    if b.counts.dom().contains(x) {
        b.counts[x]
    } else {
        0
    }
}


pub struct Bag {
    pub counts: Map<nat, nat>,
}

pub open spec fn bag_eq(b1: Bag, b2: Bag) -> bool {
    forall|x: nat| bag_count(b1, x) == bag_count(b2, x)
}


pub proof fn bag_eq_trans(b1: Bag, b2: Bag, b3: Bag)
    requires bag_eq(b1, b2), bag_eq(b2, b3)
    ensures bag_eq(b1, b3)
{
}

} // verus!