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

pub open spec fn p_get<V>(m: PartialMap<V>, k: nat) -> Option<V> {
    if m.map.dom().contains(k) {
        Some(m.map[k])
    } else {
        None
    }
}


pub proof fn p_get_empty<V>(k: nat)
    ensures p_get::<V>(p_empty(), k).is_none()
{
}

} // verus!