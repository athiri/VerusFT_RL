use vstd::prelude::*;

verus! {

pub open spec fn neg_false() -> bool {
    !false == true
}


pub proof fn verify_neg_false()
    ensures neg_false()
{
}

} // verus!