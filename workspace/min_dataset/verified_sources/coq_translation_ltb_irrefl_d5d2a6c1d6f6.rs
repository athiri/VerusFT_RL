use vstd::prelude::*;

verus! {

pub open spec fn ltb(a: nat, b: nat) -> bool {
    a < b
}


pub proof fn ltb_irrefl(a: nat)
    ensures !ltb(a, a)
{
}

} // verus!