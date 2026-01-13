use vstd::prelude::*;

verus! {

pub open spec fn disj_idemp(p: bool) -> bool {
    (p || p) == p
}


pub proof fn verify_disj_idemp(p: bool)
    ensures disj_idemp(p)
{
}

} // verus!