use vstd::prelude::*;

verus! {

pub open spec fn disj_assoc(p: bool, q: bool, r: bool) -> bool {
    ((p || q) || r) == (p || (q || r))
}


pub proof fn verify_disj_assoc(p: bool, q: bool, r: bool)
    ensures disj_assoc(p, q, r)
{
}

} // verus!