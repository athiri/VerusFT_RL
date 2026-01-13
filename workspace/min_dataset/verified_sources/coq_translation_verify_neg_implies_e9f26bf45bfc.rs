use vstd::prelude::*;

verus! {

pub open spec fn neg_implies(p: bool, q: bool) -> bool {
    !(p ==> q) == (p && !q)
}


pub proof fn verify_neg_implies(p: bool, q: bool)
    ensures neg_implies(p, q)
{
}

} // verus!