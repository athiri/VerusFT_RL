use vstd::prelude::*;

verus! {

pub open spec fn neg_elim(p: bool) -> bool {
    !!p == p
}


pub proof fn verify_neg_elim(p: bool)
    ensures neg_elim(p)
{
}

} // verus!