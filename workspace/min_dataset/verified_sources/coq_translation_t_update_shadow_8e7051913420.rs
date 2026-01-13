use vstd::prelude::*;

verus! {

pub struct TotalMap<V> {
    pub default: V,
    pub map: Map<nat, V>,
}

pub open spec fn t_get<V>(m: TotalMap<V>, k: nat) -> V {
    if m.map.dom().contains(k) {
        m.map[k]
    } else {
        m.default
    }
}

pub open spec fn t_update<V>(m: TotalMap<V>, k: nat, v: V) -> TotalMap<V> {
    TotalMap {
        default: m.default,
        map: m.map.insert(k, v),
    }
}


pub proof fn t_update_shadow<V>(m: TotalMap<V>, k: nat, v1: V, v2: V)
    ensures t_get(t_update(t_update(m, k, v1), k, v2), k) == v2
{
}

} // verus!