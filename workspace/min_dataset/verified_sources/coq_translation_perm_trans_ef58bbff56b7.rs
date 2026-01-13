use vstd::prelude::*;

verus! {

pub open spec fn bag_count(b: Bag, x: nat) -> nat {
    if b.counts.dom().contains(x) {
        b.counts[x]
    } else {
        0
    }
}


pub open spec fn bag_add(b: Bag, x: nat) -> Bag {
    Bag {
        counts: b.counts.insert(x, bag_count(b, x) + 1),
    }
}

pub open spec fn bag_eq(b1: Bag, b2: Bag) -> bool {
    forall|x: nat| bag_count(b1, x) == bag_count(b2, x)
}


pub struct Bag {
    pub counts: Map<nat, nat>,
}

pub proof fn bag_eq_trans(b1: Bag, b2: Bag, b3: Bag)
    requires bag_eq(b1, b2), bag_eq(b2, b3)
    ensures bag_eq(b1, b3)
{
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

pub open spec fn is_permutation(s1: Seq<nat>, s2: Seq<nat>) -> bool {
    bag_eq(seq_to_bag(s1), seq_to_bag(s2))
}


pub proof fn perm_trans(s1: Seq<nat>, s2: Seq<nat>, s3: Seq<nat>)
    requires is_permutation(s1, s2), is_permutation(s2, s3)
    ensures is_permutation(s1, s3)
{
    bag_eq_trans(seq_to_bag(s1), seq_to_bag(s2), seq_to_bag(s3));
}

} // verus!