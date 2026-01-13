use vstd::prelude::*;

verus! {

pub open spec fn contradiction(p: bool) -> bool {
    (p && !p) == false
}


pub proof fn verify_contradiction(p: bool)
    ensures contradiction(p)
{
}

} // verus!