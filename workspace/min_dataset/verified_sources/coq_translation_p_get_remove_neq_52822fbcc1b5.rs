use vstd::prelude::*;

verus! {

pub struct PartialMap<V> {
    pub map: Map<nat, V>,
}

pub open spec fn p_remove<V>(m: PartialMap<V>, k: nat) -> PartialMap<V> {
    PartialMap {
        map: m.map.remove(k),
    }
}

pub open spec fn p_get<V>(m: PartialMap<V>, k: nat) -> Option<V> {
    if m.map.dom().contains(k) {
        Some(m.map[k])
    } else {
        None
    }
}


pub proof fn p_get_remove_neq<V>(m: PartialMap<V>, k1: nat, k2: nat)
    requires k1 != k2
    ensures p_get(p_remove(m, k1), k2) == p_get(m, k2)
{
}

} // verus!