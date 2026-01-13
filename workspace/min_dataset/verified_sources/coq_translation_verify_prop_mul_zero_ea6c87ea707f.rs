use vstd::prelude::*;

verus! {

pub open spec fn prop_mul_zero(x: nat) -> bool {
    x * 0 == 0 && 0 * x == 0
}


pub proof fn verify_prop_mul_zero(x: nat)
    ensures prop_mul_zero(x)
{
}

} // verus!