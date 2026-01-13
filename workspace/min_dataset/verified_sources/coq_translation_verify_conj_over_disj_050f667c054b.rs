use vstd::prelude::*;

verus! {

pub open spec fn conj_over_disj(p: bool, q: bool, r: bool) -> bool {
    (p && (q || r)) == ((p && q) || (p && r))
}


pub proof fn verify_conj_over_disj(p: bool, q: bool, r: bool)
    ensures conj_over_disj(p, q, r)
{
}

} // verus!