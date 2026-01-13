use vstd::prelude::*;

verus! {

pub open spec fn set_insert(x: nat, s: Set<nat>) -> Set<nat> {
    s.insert(x)
}

pub open spec fn set_contains(x: nat, s: Set<nat>) -> bool {
    s.contains(x)
}


pub proof fn insert_preserves(x: nat, y: nat, s: Set<nat>)
    requires x != y
    ensures set_contains(y, set_insert(x, s)) == set_contains(y, s)
{
}

} // verus!