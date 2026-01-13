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


pub proof fn ex_map_update_shadow_value(m: M, k: Key, v1: int, v2: int)
    ensures (m.insert(k, v1).insert(k, v2))[k] == v2
{
    lemma_insert_same(m.insert(k, v1), k, v2);
}

} // verus!