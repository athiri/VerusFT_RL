use vstd::prelude::*;

verus! {

pub open spec fn conj_identity(p: bool) -> bool {
    (p && true) == p
}


pub proof fn verify_conj_identity(p: bool)
    ensures conj_identity(p)
{
}

} // verus!