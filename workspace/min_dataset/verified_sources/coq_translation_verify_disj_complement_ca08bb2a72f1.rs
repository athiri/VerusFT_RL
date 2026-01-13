use vstd::prelude::*;

verus! {

pub open spec fn disj_complement(p: bool) -> bool {
    (p || !p) == true
}


pub proof fn verify_disj_complement(p: bool)
    ensures disj_complement(p)
{
}

} // verus!