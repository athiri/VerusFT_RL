use vstd::prelude::*;

verus! {

pub open spec fn conj_assoc(p: bool, q: bool, r: bool) -> bool {
    ((p && q) && r) == (p && (q && r))
}


pub proof fn verify_conj_assoc(p: bool, q: bool, r: bool)
    ensures conj_assoc(p, q, r)
{
}

} // verus!