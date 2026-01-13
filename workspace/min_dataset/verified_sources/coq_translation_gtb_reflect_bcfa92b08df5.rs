use vstd::prelude::*;

verus! {

pub open spec fn gtb(x: nat, y: nat) -> bool {
    x > y
}


pub proof fn gtb_reflect(x: nat, y: nat)
    ensures gtb(x, y) <==> x > y
{
}

} // verus!