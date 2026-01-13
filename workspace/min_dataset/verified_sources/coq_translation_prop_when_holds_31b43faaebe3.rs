use vstd::prelude::*;

verus! {

pub open spec fn prop_when(condition: bool, property: bool) -> bool {
    condition ==> property
}


pub proof fn prop_when_holds(p: bool)
    requires p
    ensures prop_when(true, p)
{
}

} // verus!