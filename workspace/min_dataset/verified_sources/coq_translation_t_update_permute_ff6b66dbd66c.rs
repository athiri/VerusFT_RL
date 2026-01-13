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


pub proof fn t_update_permute<V>(m: TotalMap<V>, k1: nat, v1: V, k2: nat, v2: V)
    requires k1 != k2
    ensures forall|k: nat|
        t_get(t_update(t_update(m, k1, v1), k2, v2), k) ==
        t_get(t_update(t_update(m, k2, v2), k1, v1), k)
{
}

} // verus!