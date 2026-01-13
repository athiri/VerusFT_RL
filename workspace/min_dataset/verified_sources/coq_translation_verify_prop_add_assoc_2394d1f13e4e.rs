use vstd::prelude::*;

verus! {

pub open spec fn prop_add_assoc(x: nat, y: nat, z: nat) -> bool {
    (x + y) + z == x + (y + z)
}


pub proof fn verify_prop_add_assoc(x: nat, y: nat, z: nat)
    ensures prop_add_assoc(x, y, z)
{
}

} // verus!