use vstd::prelude::*;

verus! {
use vstd::map::{axiom_map_insert_same, axiom_map_insert_domain, axiom_map_insert_different};


pub open spec fn t_update(m: TotalMap, k: Key, v: int) -> TotalMap {
    m.insert(k, v)
}


pub type Key = nat;

pub type TotalMap = Map<Key, int>;

pub open spec fn t_apply(m: TotalMap, default: int, k: Key) -> int {
    if m.dom().contains(k) { m[k] } else { default }
}


pub proof fn ex1_update_eq(m: TotalMap, default: int, k: Key, v: int)
    ensures t_apply(t_update(m, k, v), default, k) == v
{
    axiom_map_insert_domain(m, k, v);
    assert(t_update(m, k, v).dom().contains(k));
    axiom_map_insert_same(m, k, v);
    assert(t_apply(t_update(m, k, v), default, k) == t_update(m, k, v)[k]);
}

} // verus!