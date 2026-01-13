use vstd::prelude::*;

verus! {

pub open spec fn prop_vacuous(p: bool) -> bool {
    false ==> p
}


pub proof fn verify_vacuous(p: bool)
    ensures prop_vacuous(p)
{
}

} // verus!