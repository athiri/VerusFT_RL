use vstd::prelude::*;

verus! {

pub open spec fn set_contains(x: nat, s: Set<nat>) -> bool {
    s.contains(x)
}

pub open spec fn set_remove(x: nat, s: Set<nat>) -> Set<nat> {
    s.remove(x)
}


pub proof fn remove_not_contains(x: nat, s: Set<nat>)
    ensures !set_contains(x, set_remove(x, s))
{
}

} // verus!