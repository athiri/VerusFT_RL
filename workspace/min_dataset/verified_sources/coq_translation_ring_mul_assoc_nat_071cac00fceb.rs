use vstd::prelude::*;

verus! {

pub open spec fn ring_mul_nat(a: nat, b: nat) -> nat {
    a * b
}


pub proof fn ring_mul_assoc_nat(a: nat, b: nat, c: nat)
    ensures ring_mul_nat(ring_mul_nat(a, b), c) == ring_mul_nat(a, ring_mul_nat(b, c))
{
    assert((a * b) * c == a * (b * c)) by(nonlinear_arith);
}

} // verus!