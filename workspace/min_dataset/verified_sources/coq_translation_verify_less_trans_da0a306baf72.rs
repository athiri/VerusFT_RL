use vstd::prelude::*;

verus! {

pub open spec fn prop_less_trans(x: nat, y: nat, z: nat) -> bool {
    (x < y && y < z) ==> (x < z)
}


pub proof fn verify_less_trans(x: nat, y: nat, z: nat)
    ensures prop_less_trans(x, y, z)
{
}

} // verus!