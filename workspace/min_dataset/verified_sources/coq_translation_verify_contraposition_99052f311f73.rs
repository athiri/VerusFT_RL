use vstd::prelude::*;

verus! {

pub open spec fn contraposition(p: bool, q: bool) -> bool {
    (p ==> q) == (!q ==> !p)
}


pub proof fn verify_contraposition(p: bool, q: bool)
    ensures contraposition(p, q)
{
}

} // verus!