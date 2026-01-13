use vstd::prelude::*;

verus! {

pub open spec fn construct_double(n: nat) -> nat {
    n * 2
}


pub proof fn verify_construct_double(n: nat)
    ensures construct_double(n) == n * 2
{
}

} // verus!