use vstd::prelude::*;

verus! {

pub open spec fn max(a: nat, b: nat) -> nat {
    if a >= b { a } else { b }
}


pub proof fn max_ge_right(a: nat, b: nat)
    ensures max(a, b) >= b
{
}

} // verus!