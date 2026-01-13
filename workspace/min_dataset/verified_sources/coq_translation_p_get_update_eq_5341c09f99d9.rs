use vstd::prelude::*;

verus! {

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


pub proof fn p_get_update_eq<V>(m: PartialMap<V>, k: nat, v: V)
    ensures p_get(p_update(m, k, v), k) == Some(v)
{
}

} // verus!