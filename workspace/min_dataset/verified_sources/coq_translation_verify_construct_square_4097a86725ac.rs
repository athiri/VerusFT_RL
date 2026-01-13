use vstd::prelude::*;

verus! {

pub open spec fn construct_square(n: nat) -> nat {
    n * n
}


pub proof fn verify_construct_square(n: nat)
    ensures construct_square(n) == n * n
{
}

} // verus!