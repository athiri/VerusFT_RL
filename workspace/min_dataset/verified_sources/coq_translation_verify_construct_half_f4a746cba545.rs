use vstd::prelude::*;

verus! {

pub open spec fn construct_half(n: nat) -> nat
    recommends n % 2 == 0
{
    n / 2
}


pub proof fn verify_construct_half(n: nat)
    requires n % 2 == 0
    ensures construct_half(n) * 2 == n
{
}

} // verus!