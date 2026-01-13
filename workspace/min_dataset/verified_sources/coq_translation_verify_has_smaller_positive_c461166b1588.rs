use vstd::prelude::*;

verus! {

pub open spec fn has_smaller_positive(n: nat) -> bool {
    n > 1  // 1 exists and is smaller
}


pub proof fn verify_has_smaller_positive(n: nat)
    requires n > 1
    ensures has_smaller_positive(n)
{
}

} // verus!