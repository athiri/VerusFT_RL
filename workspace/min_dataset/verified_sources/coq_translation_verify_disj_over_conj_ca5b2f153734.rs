use vstd::prelude::*;

verus! {

pub open spec fn disj_over_conj(p: bool, q: bool, r: bool) -> bool {
    (p || (q && r)) == ((p || q) && (p || r))
}


pub proof fn verify_disj_over_conj(p: bool, q: bool, r: bool)
    ensures disj_over_conj(p, q, r)
{
}

} // verus!