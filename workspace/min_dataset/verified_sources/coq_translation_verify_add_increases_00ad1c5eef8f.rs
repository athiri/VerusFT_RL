use vstd::prelude::*;

verus! {

pub open spec fn prop_add_increases(x: nat, y: nat) -> bool {
    (x > 0 && y > 0) ==> (x + y > x)
}


pub proof fn verify_add_increases(x: nat, y: nat)
    ensures prop_add_increases(x, y)
{
}

} // verus!