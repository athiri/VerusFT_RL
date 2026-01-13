use vstd::prelude::*;

verus! {

pub open spec fn has_zero_in_range(bound: nat) -> bool {
    bound > 0
}


pub proof fn verify_has_zero_in_range(bound: nat)
    requires bound > 0
    ensures has_zero_in_range(bound)
{
}

} // verus!