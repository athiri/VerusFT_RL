use vstd::prelude::*;

verus! {

pub open spec fn monoid_nat_mul_identity() -> nat {
    1  // Different from default_nat for multiplication
}


pub proof fn monoid_nat_mul_laws_identity_right(x: nat)
    ensures x * monoid_nat_mul_identity() == x
{
    assert(x * 1 == x);
}

} // verus!