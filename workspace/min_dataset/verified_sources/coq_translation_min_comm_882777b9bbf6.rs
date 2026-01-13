use vstd::prelude::*;

verus! {

pub open spec fn min(a: nat, b: nat) -> nat {
    if a <= b { a } else { b }
}


pub proof fn min_comm(a: nat, b: nat)
    ensures min(a, b) == min(b, a)
{
}

} // verus!