use vstd::prelude::*;

verus! {
use vstd::map::{axiom_map_insert_same, axiom_map_insert_domain, axiom_map_insert_different};


pub type Key = nat;

pub type M = Map<Key, int>;


pub proof fn lemma_insert_same(m: M, k: Key, v: int)
    ensures m.insert(k, v)[k] == v
{
    axiom_map_insert_same(m, k, v);
}

} // verus!