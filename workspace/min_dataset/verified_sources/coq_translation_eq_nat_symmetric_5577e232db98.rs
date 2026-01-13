use vstd::prelude::*;

verus! {

pub open spec fn eq_nat(a: nat, b: nat) -> bool {
    a == b
}


pub proof fn eq_nat_symmetric(x: nat, y: nat)
    requires eq_nat(x, y)
    ensures eq_nat(y, x)
{
}

} // verus!