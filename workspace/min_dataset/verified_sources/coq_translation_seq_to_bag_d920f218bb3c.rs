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

pub open spec fn bag_add(b: Bag, x: nat) -> Bag {
    Bag {
        counts: b.counts.insert(x, bag_count(b, x) + 1),
    }
}


pub open spec fn seq_to_bag(s: Seq<nat>) -> Bag
    decreases s.len()
{
    if s.len() == 0 {
        Bag { counts: Map::empty() }
    } else {
        bag_add(seq_to_bag(s.skip(1)), s[0])
    }
}

} // verus!