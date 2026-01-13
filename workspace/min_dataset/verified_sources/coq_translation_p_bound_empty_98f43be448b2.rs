use vstd::prelude::*;

verus! {

pub struct PartialMap<V> {
    pub map: Map<nat, V>,
}

pub open spec fn p_empty<V>() -> PartialMap<V> {
    PartialMap {
        map: Map::<nat, V>::empty(),
    }
}

pub open spec fn p_bound<V>(m: PartialMap<V>, k: nat) -> bool {
    m.map.dom().contains(k)
}


pub proof fn p_bound_empty<V>(k: nat)
    ensures !p_bound::<V>(p_empty(), k)
{
}

} // verus!