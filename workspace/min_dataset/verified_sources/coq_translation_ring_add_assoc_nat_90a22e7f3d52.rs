use vstd::prelude::*;

verus! {

pub open spec fn ring_add_nat(a: nat, b: nat) -> nat {
    (a + b) as nat
}


pub proof fn ring_add_assoc_nat(a: nat, b: nat, c: nat)
    ensures ring_add_nat(ring_add_nat(a, b), c) == ring_add_nat(a, ring_add_nat(b, c))
{
    assert(((a + b) + c) as nat == (a + (b + c)) as nat);
}

} // verus!