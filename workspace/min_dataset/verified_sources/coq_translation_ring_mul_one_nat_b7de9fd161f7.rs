use vstd::prelude::*;

verus! {

pub open spec fn ring_one_nat() -> nat {
    1
}

pub open spec fn ring_mul_nat(a: nat, b: nat) -> nat {
    a * b
}


pub proof fn ring_mul_one_nat(a: nat)
    ensures ring_mul_nat(ring_one_nat(), a) == a,
            ring_mul_nat(a, ring_one_nat()) == a
{
    assert(1 * a == a);
    assert(a * 1 == a);
}

} // verus!