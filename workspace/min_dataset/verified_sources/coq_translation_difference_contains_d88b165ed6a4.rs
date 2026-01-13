use vstd::prelude::*;

verus! {

pub open spec fn set_difference(s1: Set<nat>, s2: Set<nat>) -> Set<nat> {
    s1.difference(s2)
}

pub open spec fn set_contains(x: nat, s: Set<nat>) -> bool {
    s.contains(x)
}


pub proof fn difference_contains(x: nat, s1: Set<nat>, s2: Set<nat>)
    ensures set_contains(x, set_difference(s1, s2)) ==
            (set_contains(x, s1) && !set_contains(x, s2))
{
}

} // verus!