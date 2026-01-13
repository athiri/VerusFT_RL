use vstd::prelude::*;

verus! {

pub open spec fn max_nat(a: nat, b: nat) -> nat {
    if a >= b { a } else { b }
}


pub proof fn max_ge_both(a: nat, b: nat)
    ensures max_nat(a, b) >= a && max_nat(a, b) >= b
{
}

} // verus!