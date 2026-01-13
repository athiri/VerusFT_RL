use vstd::prelude::*;

verus! {

pub open spec fn ring_zero_nat() -> nat {
    0
}

pub open spec fn ring_add_nat(a: nat, b: nat) -> nat {
    (a + b) as nat
}


pub proof fn ring_add_zero_nat(a: nat)
    ensures ring_add_nat(ring_zero_nat(), a) == a,
            ring_add_nat(a, ring_zero_nat()) == a
{
    assert((0 + a) as nat == a);
    assert((a + 0) as nat == a);
}

} // verus!