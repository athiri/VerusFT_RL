use vstd::prelude::*;

verus! {

pub open spec fn prop_add_comm(x: nat, y: nat) -> bool {
    x + y == y + x
}


pub proof fn verify_prop_add_comm(x: nat, y: nat)
    ensures prop_add_comm(x, y)
{
}

} // verus!