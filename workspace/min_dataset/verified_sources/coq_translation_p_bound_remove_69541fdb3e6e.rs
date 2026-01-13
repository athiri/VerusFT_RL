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

pub open spec fn p_bound<V>(m: PartialMap<V>, k: nat) -> bool {
    m.map.dom().contains(k)
}


pub proof fn p_bound_remove<V>(m: PartialMap<V>, k: nat)
    ensures !p_bound(p_remove(m, k), k)
{
}

} // verus!