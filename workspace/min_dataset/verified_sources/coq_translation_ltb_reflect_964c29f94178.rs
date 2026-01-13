use vstd::prelude::*;

verus! {

pub open spec fn ltb(x: nat, y: nat) -> bool {
    x < y
}


pub proof fn ltb_reflect(x: nat, y: nat)
    ensures ltb(x, y) <==> x < y
{
}

} // verus!