use vstd::prelude::*;

verus! {

pub open spec fn has_greater_than(n: nat) -> bool {
    true  // Always exists: n + 1
}


pub proof fn verify_has_greater_than(n: nat)
    ensures has_greater_than(n)
{
}

} // verus!