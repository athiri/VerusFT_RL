use vstd::prelude::*;

verus! {

pub open spec fn ring_add_nat(a: nat, b: nat) -> nat {
    (a + b) as nat
}

pub open spec fn ring_mul_nat(a: nat, b: nat) -> nat {
    a * b
}


pub proof fn ring_distrib_nat(a: nat, b: nat, c: nat)
    ensures ring_mul_nat(a, ring_add_nat(b, c)) == ring_add_nat(ring_mul_nat(a, b), ring_mul_nat(a, c))
{
    assert(a * ((b + c) as nat) == ((a * b) + (a * c)) as nat) by(nonlinear_arith);
}

} // verus!