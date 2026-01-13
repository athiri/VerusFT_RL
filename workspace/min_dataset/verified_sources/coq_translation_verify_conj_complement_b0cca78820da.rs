use vstd::prelude::*;

verus! {

pub open spec fn conj_complement(p: bool) -> bool {
    (p && !p) == false
}


pub proof fn verify_conj_complement(p: bool)
    ensures conj_complement(p)
{
}

} // verus!