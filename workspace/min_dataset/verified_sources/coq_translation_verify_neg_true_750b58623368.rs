use vstd::prelude::*;

verus! {

pub open spec fn neg_true() -> bool {
    !true == false
}


pub proof fn verify_neg_true()
    ensures neg_true()
{
}

} // verus!