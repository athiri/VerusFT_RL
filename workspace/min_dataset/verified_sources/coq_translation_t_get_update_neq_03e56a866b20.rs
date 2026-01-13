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


pub proof fn t_get_update_neq<V>(m: TotalMap<V>, k1: nat, k2: nat, v: V)
    requires k1 != k2
    ensures t_get(t_update(m, k1, v), k2) == t_get(m, k2)
{
}

} // verus!