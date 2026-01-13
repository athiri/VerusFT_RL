use vstd::prelude::*;

verus! {

pub open spec fn eq_nat(a: nat, b: nat) -> bool {
    a == b
}


pub proof fn eq_nat_sound(x: nat, y: nat)
    requires eq_nat(x, y)
    ensures x == y
{
}

} // verus!