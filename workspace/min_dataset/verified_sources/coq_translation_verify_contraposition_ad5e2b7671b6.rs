use vstd::prelude::*;

verus! {

pub open spec fn prop_contraposition(p: bool, q: bool) -> bool {
    (p ==> q) <==> (!q ==> !p)
}


pub proof fn verify_contraposition(p: bool, q: bool)
    ensures prop_contraposition(p, q)
{
}

} // verus!