use vstd::prelude::*;

verus! {

pub open spec fn min_nat(a: nat, b: nat) -> nat {
    if a <= b { a } else { b }
}


pub proof fn min_le_both(a: nat, b: nat)
    ensures min_nat(a, b) <= a && min_nat(a, b) <= b
{
}

} // verus!