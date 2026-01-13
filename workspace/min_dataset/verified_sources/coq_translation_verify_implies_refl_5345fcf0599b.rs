use vstd::prelude::*;

verus! {

pub open spec fn prop_implies_refl(p: bool) -> bool {
    p ==> p
}


pub proof fn verify_implies_refl(p: bool)
    ensures prop_implies_refl(p)
{
}

} // verus!