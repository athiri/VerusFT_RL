use vstd::prelude::*;

verus! {
use vstd::map::{axiom_map_insert_same, axiom_map_insert_domain, axiom_map_insert_different};


pub proof fn ex1_update_eq(m: TotalMap, default: int, k: Key, v: int)
    ensures t_apply(t_update(m, k, v), default, k) == v
{
    axiom_map_insert_domain(m, k, v);
    assert(t_update(m, k, v).dom().contains(k));
    axiom_map_insert_same(m, k, v);
    assert(t_apply(t_update(m, k, v), default, k) == t_update(m, k, v)[k]);
}

pub proof fn ex2_update_neq(m: TotalMap, default: int, k1: Key, k2: Key, v: int)
    requires k2 != k1,
    ensures t_apply(t_update(m, k1, v), default, k2) == t_apply(m, default, k2)
{
    axiom_map_insert_domain(m, k1, v);
    if m.dom().contains(k2) {
        // k2 was already mapped; insert at k1 (k1!=k2) preserves value at k2
        assert(t_update(m, k1, v).dom().contains(k2));
        axiom_map_insert_different(m, k2, k1, v);
        assert(t_update(m, k1, v)[k2] == m[k2]);
        assert(t_apply(t_update(m, k1, v), default, k2) == t_update(m, k1, v)[k2]);
        assert(t_apply(m, default, k2) == m[k2]);
    } else {
        // k2 unmapped before; inserting at different key doesn't add k2
        assert(!t_update(m, k1, v).dom().contains(k2));
        assert(t_apply(t_update(m, k1, v), default, k2) == default);
        assert(t_apply(m, default, k2) == default);
    }
}


pub open spec fn t_apply(m: TotalMap, default: int, k: Key) -> int {
    if m.dom().contains(k) { m[k] } else { default }
}

pub open spec fn t_update(m: TotalMap, k: Key, v: int) -> TotalMap {
    m.insert(k, v)
}


pub type Key = nat;

pub type TotalMap = Map<Key, int>;


pub proof fn ex5_update_comm(m: TotalMap, default: int, k1: Key, v1: int, k2: Key, v2: int)
    requires k1 != k2,
    ensures forall|x: Key| t_apply(t_update(t_update(m, k1, v1), k2, v2), default, x)
        == t_apply(t_update(t_update(m, k2, v2), k1, v1), default, x)
{
    assert forall|x: Key| t_apply(t_update(t_update(m, k1, v1), k2, v2), default, x)
        == t_apply(t_update(t_update(m, k2, v2), k1, v1), default, x)
    by {
        if x == k1 {
            // Left: update at k2 doesn't affect k1
            ex2_update_neq(t_update(m, k1, v1), default, k2, k1, v2);
            ex1_update_eq(t_update(m, k2, v2), default, k1, v1);
        } else if x == k2 {
            ex1_update_eq(t_update(m, k1, v1), default, k2, v2);
            ex2_update_neq(t_update(m, k2, v2), default, k1, k2, v1);
        } else {
            ex2_update_neq(t_update(m, k1, v1), default, k2, x, v2);
            ex2_update_neq(t_update(m, k2, v2), default, k1, x, v1);
        }
    };
}

} // verus!