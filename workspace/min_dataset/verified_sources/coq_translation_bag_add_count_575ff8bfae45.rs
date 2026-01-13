use vstd::prelude::*;

verus! {

pub struct Bag {
    pub counts: Map<nat, nat>,
}

pub open spec fn bag_count(b: Bag, x: nat) -> nat {
    if b.counts.contains_key(x) {
        b.counts[x]
    } else {
        0
    }
}

pub open spec fn bag_add(b: Bag, x: nat) -> Bag {
    Bag { counts: b.counts.insert(x, bag_count(b, x) + 1) }
}

pub proof fn bag_add_count(b: Bag, x: nat, y: nat)
    ensures bag_count(bag_add(b, x), y) == if x == y { bag_count(b, y) + 1 } else { bag_count(b, y) }
{
}

} // verus!
