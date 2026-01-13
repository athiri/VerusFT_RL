use vstd::prelude::*;

verus! {

pub open spec fn prop_true() -> bool {
    true
}


pub proof fn verify_prop_true()
    ensures prop_true()
{
}

} // verus!