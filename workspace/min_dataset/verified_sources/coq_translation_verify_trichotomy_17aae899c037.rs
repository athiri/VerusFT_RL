use vstd::prelude::*;

verus! {

pub open spec fn trichotomy(x: nat, y: nat) -> bool {
    x < y || x == y || x > y
}


pub proof fn verify_trichotomy(x: nat, y: nat)
    ensures trichotomy(x, y)
{
}

} // verus!