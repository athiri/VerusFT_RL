use vstd::prelude::*;

verus! {
use vstd::map::{axiom_map_insert_same, axiom_map_insert_domain, axiom_map_insert_different};


pub type Key = nat;

pub type M = Map<Key, int>;


pub proof fn lemma_insert_different(m: M, k1: Key, k2: Key, v2: int)
    requires k1 != k2
    ensures m.insert(k2, v2)[k1] == m[k1]
{
    axiom_map_insert_different(m, k1, k2, v2);
}

} // verus!