use vstd::prelude::*;

verus! {

pub open spec fn set_contains(x: nat, s: Set<nat>) -> bool {
    s.contains(x)
}

pub open spec fn set_empty() -> Set<nat> {
    Set::empty()
}


pub proof fn empty_no_elements(x: nat)
    ensures !set_contains(x, set_empty())
{
}

} // verus!