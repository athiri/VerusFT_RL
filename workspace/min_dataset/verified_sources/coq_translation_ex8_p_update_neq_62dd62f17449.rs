use vstd::prelude::*;

verus! {
use vstd::map::{axiom_map_insert_same, axiom_map_insert_domain, axiom_map_insert_different};


pub open spec fn p_update(m: PartialMap, k: Key, v: int) -> PartialMap {
    m.insert(k, v)
}

pub open spec fn p_apply(m: PartialMap, k: Key) -> Option<int> {
    if m.dom().contains(k) { Option::Some(m[k]) } else { Option::<int>::None }
}


pub type Key = nat;

pub type PartialMap = Map<Key, int>;


pub proof fn ex8_p_update_neq(m: PartialMap, k1: Key, k2: Key, v: int)
    requires k2 != k1,
    ensures p_apply(p_update(m, k1, v), k2) == p_apply(m, k2)
{
    axiom_map_insert_domain(m, k1, v);
    if m.dom().contains(k2) {
        assert(p_update(m, k1, v).dom().contains(k2));
        axiom_map_insert_different(m, k2, k1, v);
    } else {
        assert(!p_update(m, k1, v).dom().contains(k2));
    }
}

} // verus!