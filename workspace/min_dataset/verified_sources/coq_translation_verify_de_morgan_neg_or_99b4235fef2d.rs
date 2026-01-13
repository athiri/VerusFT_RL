use vstd::prelude::*;

verus! {

pub open spec fn de_morgan_neg_or(p: bool, q: bool) -> bool {
    !(p || q) == (!p && !q)
}


pub proof fn verify_de_morgan_neg_or(p: bool, q: bool)
    ensures de_morgan_neg_or(p, q)
{
}

} // verus!