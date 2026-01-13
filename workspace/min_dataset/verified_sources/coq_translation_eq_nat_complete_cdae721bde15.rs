use vstd::prelude::*;

verus! {

pub open spec fn eq_nat(a: nat, b: nat) -> bool {
    a == b
}


pub proof fn eq_nat_complete(x: nat, y: nat)
    requires x == y
    ensures eq_nat(x, y)
{
}

} // verus!