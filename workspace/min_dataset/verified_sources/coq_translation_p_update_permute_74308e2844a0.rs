use vstd::prelude::*;

verus! {

pub proof fn p_get_update_neq<V>(m: PartialMap<V>, k1: nat, k2: nat, v: V)
    requires k1 != k2
    ensures p_get(p_update(m, k1, v), k2) == p_get(m, k2)
{
}

pub proof fn p_get_update_eq<V>(m: PartialMap<V>, k: nat, v: V)
    ensures p_get(p_update(m, k, v), k) == Some(v)
{
}


pub struct PartialMap<V> {
    pub map: Map<nat, V>,
}

pub open spec fn p_update<V>(m: PartialMap<V>, k: nat, v: V) -> PartialMap<V> {
    PartialMap {
        map: m.map.insert(k, v),
    }
}

pub open spec fn p_get<V>(m: PartialMap<V>, k: nat) -> Option<V> {
    if m.map.dom().contains(k) {
        Some(m.map[k])
    } else {
        None
    }
}


pub proof fn p_update_permute<V>(m: PartialMap<V>, k1: nat, v1: V, k2: nat, v2: V, k: nat)
    requires k1 != k2
    ensures p_get(p_update(p_update(m, k1, v1), k2, v2), k) ==
            p_get(p_update(p_update(m, k2, v2), k1, v1), k)
{
    if k == k1 {
        p_get_update_eq(p_update(m, k2, v2), k1, v1);
        p_get_update_neq(p_update(m, k1, v1), k1, k2, v2);
        p_get_update_eq(m, k1, v1);
    } else if k == k2 {
        p_get_update_eq(p_update(m, k1, v1), k2, v2);
        p_get_update_neq(p_update(m, k2, v2), k2, k1, v1);
        p_get_update_eq(m, k2, v2);
    } else {
        p_get_update_neq(p_update(m, k1, v1), k, k2, v2);
        p_get_update_neq(m, k, k1, v1);
        p_get_update_neq(p_update(m, k2, v2), k, k1, v1);
        p_get_update_neq(m, k, k2, v2);
    }
}

} // verus!