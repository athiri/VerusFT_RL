use vstd::prelude::*;

verus! {

pub open spec fn prop_mul_zero(x: nat, y: nat) -> bool {
    (x == 0) ==> (x * y == 0)
}


pub proof fn verify_mul_zero(x: nat, y: nat)
    ensures prop_mul_zero(x, y)
{
}

} // verus!