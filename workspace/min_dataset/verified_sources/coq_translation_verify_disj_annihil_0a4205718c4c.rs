use vstd::prelude::*;

verus! {

pub open spec fn disj_annihil(p: bool) -> bool {
    (p || true) == true
}


pub proof fn verify_disj_annihil(p: bool)
    ensures disj_annihil(p)
{
}

} // verus!