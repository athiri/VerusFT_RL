use vstd::prelude::*;

verus! {

pub open spec fn eq_nat(a: nat, b: nat) -> bool {
    a == b
}


pub proof fn eq_nat_reflexive(x: nat)
    ensures eq_nat(x, x)
{
}

} // verus!