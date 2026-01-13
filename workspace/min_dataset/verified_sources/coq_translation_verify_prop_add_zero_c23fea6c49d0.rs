use vstd::prelude::*;

verus! {

pub open spec fn prop_add_zero(x: nat) -> bool {
    x + 0 == x && 0 + x == x
}


pub proof fn verify_prop_add_zero(x: nat)
    ensures prop_add_zero(x)
{
}

} // verus!