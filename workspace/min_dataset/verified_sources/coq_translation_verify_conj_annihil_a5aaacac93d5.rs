use vstd::prelude::*;

verus! {

pub open spec fn conj_annihil(p: bool) -> bool {
    (p && false) == false
}


pub proof fn verify_conj_annihil(p: bool)
    ensures conj_annihil(p)
{
}

} // verus!