use vstd::prelude::*;

verus! {

pub open spec fn max(a: nat, b: nat) -> nat {
    if a >= b { a } else { b }
}


pub proof fn max_assoc(a: nat, b: nat, c: nat)
    ensures max(max(a, b), c) == max(a, max(b, c))
{
}

} // verus!