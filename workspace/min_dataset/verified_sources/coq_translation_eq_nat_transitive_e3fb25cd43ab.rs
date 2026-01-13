use vstd::prelude::*;

verus! {

pub open spec fn eq_nat(a: nat, b: nat) -> bool {
    a == b
}


pub proof fn eq_nat_transitive(x: nat, y: nat, z: nat)
    requires eq_nat(x, y), eq_nat(y, z)
    ensures eq_nat(x, z)
{
}

} // verus!