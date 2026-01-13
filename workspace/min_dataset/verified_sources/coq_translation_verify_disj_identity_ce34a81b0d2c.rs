use vstd::prelude::*;

verus! {

pub open spec fn disj_identity(p: bool) -> bool {
    (p || false) == p
}


pub proof fn verify_disj_identity(p: bool)
    ensures disj_identity(p)
{
}

} // verus!