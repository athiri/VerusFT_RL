use vstd::prelude::*;

verus! {

pub open spec fn disj_comm(p: bool, q: bool) -> bool {
    (p || q) == (q || p)
}


pub proof fn verify_disj_comm(p: bool, q: bool)
    ensures disj_comm(p, q)
{
}

} // verus!