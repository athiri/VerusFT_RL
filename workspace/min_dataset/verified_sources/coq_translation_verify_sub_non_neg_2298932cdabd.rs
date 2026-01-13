use vstd::prelude::*;

verus! {

pub open spec fn prop_sub_non_neg(x: nat, y: nat) -> bool {
    (x >= y) ==> (x - y >= 0)
}


pub proof fn verify_sub_non_neg(x: nat, y: nat)
    ensures prop_sub_non_neg(x, y)
{
}

} // verus!