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


pub proof fn bag_eq_refl(b: Bag)
    ensures bag_eq(b, b)
{
}

} // verus!