use vstd::prelude::*;

verus! {

pub struct Multiset {
    pub counts: Map<nat, nat>,
}

impl Multiset {
    pub open spec fn empty() -> Multiset {
        Multiset { counts: Map::empty() }
    }
    
    pub open spec fn singleton(x: nat) -> Multiset {
        Multiset { counts: Map::empty().insert(x, 1) }
    }
    
    pub open spec fn count(self, x: nat) -> nat {
        if self.counts.contains_key(x) {
            self.counts[x]
        } else {
            0
        }
    }
    
    pub open spec fn add(self, x: nat) -> Multiset {
        Multiset { counts: self.counts.insert(x, self.count(x) + 1) }
    }
}

pub proof fn multiset_singleton_count(x: nat, y: nat)
    ensures Multiset::singleton(x).count(y) == if x == y { 1nat } else { 0nat }
{}

} // verus!
