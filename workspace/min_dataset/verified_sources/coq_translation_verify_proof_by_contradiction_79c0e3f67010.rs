use vstd::prelude::*;

verus! {

pub open spec fn proof_by_contradiction(p: bool) -> bool {
    (!p ==> false) ==> p
}


pub proof fn verify_proof_by_contradiction(p: bool)
    ensures proof_by_contradiction(p)
{
}

} // verus!