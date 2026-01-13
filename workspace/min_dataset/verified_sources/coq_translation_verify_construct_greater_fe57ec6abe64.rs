use vstd::prelude::*;

verus! {

pub open spec fn construct_greater(n: nat) -> nat {
    n + 1
}


pub proof fn verify_construct_greater(n: nat)
    ensures construct_greater(n) > n
{
}

} // verus!