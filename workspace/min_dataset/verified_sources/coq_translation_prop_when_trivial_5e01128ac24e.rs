use vstd::prelude::*;

verus! {

pub open spec fn prop_when(condition: bool, property: bool) -> bool {
    condition ==> property
}


pub proof fn prop_when_trivial(p: bool)
    ensures prop_when(false, p)
{
}

} // verus!