use vstd::prelude::*;

verus! {

pub open spec fn prop_true_consequent(p: bool) -> bool {
    p ==> true
}


pub proof fn verify_true_consequent(p: bool)
    ensures prop_true_consequent(p)
{
}

} // verus!