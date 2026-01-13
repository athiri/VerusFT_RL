use vstd::prelude::*;

verus! {

pub open spec fn prop_mul_comm(x: nat, y: nat) -> bool {
    x * y == y * x
}


pub proof fn verify_prop_mul_comm(x: nat, y: nat)
    ensures prop_mul_comm(x, y)
{
}

} // verus!