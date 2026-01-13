use vstd::prelude::*;

verus! {

pub open spec fn has_odd() -> bool {
    true  // 1 is odd
}


pub proof fn verify_has_odd()
    ensures has_odd()
{
}

} // verus!