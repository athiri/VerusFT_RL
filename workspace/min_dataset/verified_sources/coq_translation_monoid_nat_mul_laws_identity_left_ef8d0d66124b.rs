use vstd::prelude::*;

verus! {

pub open spec fn monoid_nat_mul_identity() -> nat {
    1  // Different from default_nat for multiplication
}


pub proof fn monoid_nat_mul_laws_identity_left(x: nat)
    ensures monoid_nat_mul_identity() * x == x
{
    assert(1 * x == x);
}

} // verus!