use vstd::prelude::*;

verus! {

pub open spec fn ring_add_nat(a: nat, b: nat) -> nat {
    (a + b) as nat
}


pub proof fn ring_add_comm_nat(a: nat, b: nat)
    ensures ring_add_nat(a, b) == ring_add_nat(b, a)
{
    assert((a + b) as nat == (b + a) as nat);
}

} // verus!