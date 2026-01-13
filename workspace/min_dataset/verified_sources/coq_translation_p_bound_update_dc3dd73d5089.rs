use vstd::prelude::*;

verus! {

pub struct PartialMap<V> {
    pub map: Map<nat, V>,
}

pub open spec fn p_bound<V>(m: PartialMap<V>, k: nat) -> bool {
    m.map.dom().contains(k)
}

pub open spec fn p_update<V>(m: PartialMap<V>, k: nat, v: V) -> PartialMap<V> {
    PartialMap {
        map: m.map.insert(k, v),
    }
}


pub proof fn p_bound_update<V>(m: PartialMap<V>, k: nat, v: V)
    ensures p_bound(p_update(m, k, v), k)
{
}

} // verus!