use vstd::prelude::*;

verus! {

pub open spec fn parity(x: nat) -> bool {
    x % 2 == 0 || x % 2 == 1
}


pub proof fn verify_parity(x: nat)
    ensures parity(x)
{
}

} // verus!