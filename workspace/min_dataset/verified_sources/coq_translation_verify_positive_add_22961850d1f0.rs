use vstd::prelude::*;

verus! {

pub open spec fn prop_positive_add(x: nat) -> bool {
    (x > 0) ==> (x + 1 > 1)
}


pub proof fn verify_positive_add(x: nat)
    ensures prop_positive_add(x)
{
}

} // verus!