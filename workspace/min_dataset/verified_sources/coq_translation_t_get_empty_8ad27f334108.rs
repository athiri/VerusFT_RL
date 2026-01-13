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

pub open spec fn t_empty<V>(default: V) -> TotalMap<V> {
    TotalMap {
        default,
        map: Map::<nat, V>::empty(),
    }
}


pub proof fn t_get_empty<V>(default: V, k: nat)
    ensures t_get(t_empty(default), k) == default
{
}

} // verus!