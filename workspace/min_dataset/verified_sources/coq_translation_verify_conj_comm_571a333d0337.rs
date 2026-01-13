use vstd::prelude::*;

verus! {

pub open spec fn conj_comm(p: bool, q: bool) -> bool {
    (p && q) == (q && p)
}


pub proof fn verify_conj_comm(p: bool, q: bool)
    ensures conj_comm(p, q)
{
}

} // verus!