use vstd::prelude::*;

verus! {

pub open spec fn set_insert(x: nat, s: Set<nat>) -> Set<nat> {
    s.insert(x)
}

pub open spec fn set_contains(x: nat, s: Set<nat>) -> bool {
    s.contains(x)
}


pub proof fn insert_contains(x: nat, s: Set<nat>)
    ensures set_contains(x, set_insert(x, s))
{
}

} // verus!