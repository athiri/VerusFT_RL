use vstd::prelude::*;

verus! {

pub open spec fn set_contains(x: nat, s: Set<nat>) -> bool {
    s.contains(x)
}

pub open spec fn set_remove(x: nat, s: Set<nat>) -> Set<nat> {
    s.remove(x)
}


pub proof fn remove_preserves(x: nat, y: nat, s: Set<nat>)
    requires x != y
    ensures set_contains(y, set_remove(x, s)) == set_contains(y, s)
{
}

} // verus!