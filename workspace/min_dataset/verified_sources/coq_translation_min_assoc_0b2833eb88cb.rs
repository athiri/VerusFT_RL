use vstd::prelude::*;

verus! {

pub open spec fn min(a: nat, b: nat) -> nat {
    if a <= b { a } else { b }
}


pub proof fn min_assoc(a: nat, b: nat, c: nat)
    ensures min(min(a, b), c) == min(a, min(b, c))
{
}

} // verus!