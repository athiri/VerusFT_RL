use vstd::prelude::*;

verus! {
use vstd::map::{axiom_map_insert_same, axiom_map_insert_domain, axiom_map_insert_different};


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

pub proof fn ex7_p_update_eq(m: PartialMap, k: Key, v: int)
    ensures p_apply(p_update(m, k, v), k) == Option::Some(v)
{
    axiom_map_insert_domain(m, k, v);
    assert(p_update(m, k, v).dom().contains(k));
    axiom_map_insert_same(m, k, v);
    assert(p_update(m, k, v)[k] == v);
}


pub open spec fn p_apply(m: PartialMap, k: Key) -> Option<int> {
    if m.dom().contains(k) { Option::Some(m[k]) } else { Option::<int>::None }
}

pub open spec fn p_update(m: PartialMap, k: Key, v: int) -> PartialMap {
    m.insert(k, v)
}


pub type Key = nat;

pub type PartialMap = Map<Key, int>;


pub proof fn ex9_p_update_shadow(m: PartialMap, k: Key, v1: int, v2: int)
    ensures forall|x: Key| p_apply(p_update(p_update(m, k, v1), k, v2), x)
        == p_apply(p_update(m, k, v2), x)
{
    assert forall|x: Key| p_apply(p_update(p_update(m, k, v1), k, v2), x)
        == p_apply(p_update(m, k, v2), x)
    by {
        if x == k {
            ex7_p_update_eq(p_update(m, k, v1), k, v2);
            ex7_p_update_eq(m, k, v2);
        } else {
            ex8_p_update_neq(p_update(m, k, v1), k, x, v2);
            ex8_p_update_neq(m, k, x, v2);
        }
    };
}

} // verus!