use vstd::prelude::*;

verus! {

pub open spec fn conj_idemp(p: bool) -> bool {
    (p && p) == p
}


pub proof fn verify_conj_idemp(p: bool)
    ensures conj_idemp(p)
{
}

} // verus!