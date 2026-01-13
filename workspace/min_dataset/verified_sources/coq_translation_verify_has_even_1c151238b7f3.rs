use vstd::prelude::*;

verus! {

pub open spec fn has_even() -> bool {
    true  // 0 is even
}


pub proof fn verify_has_even()
    ensures has_even()
{
}

} // verus!