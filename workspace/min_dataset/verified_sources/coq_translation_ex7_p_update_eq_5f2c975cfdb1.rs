use vstd::prelude::*;

verus! {
use vstd::map::{axiom_map_insert_same, axiom_map_insert_domain, axiom_map_insert_different};


pub open spec fn p_apply(m: PartialMap, k: Key) -> Option<int> {
    if m.dom().contains(k) { Option::Some(m[k]) } else { Option::<int>::None }
}

pub open spec fn p_update(m: PartialMap, k: Key, v: int) -> PartialMap {
    m.insert(k, v)
}


pub type Key = nat;

pub type PartialMap = Map<Key, int>;


pub proof fn ex7_p_update_eq(m: PartialMap, k: Key, v: int)
    ensures p_apply(p_update(m, k, v), k) == Option::Some(v)
{
    axiom_map_insert_domain(m, k, v);
    assert(p_update(m, k, v).dom().contains(k));
    axiom_map_insert_same(m, k, v);
    assert(p_update(m, k, v)[k] == v);
}

} // verus!