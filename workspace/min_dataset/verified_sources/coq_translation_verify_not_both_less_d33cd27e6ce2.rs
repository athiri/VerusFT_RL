use vstd::prelude::*;

verus! {

pub open spec fn not_both_less(x: nat, y: nat) -> bool {
    !(x < y && y < x)
}


pub proof fn verify_not_both_less(x: nat, y: nat)
    ensures not_both_less(x, y)
{
}

} // verus!