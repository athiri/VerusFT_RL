use vstd::prelude::*;

verus! {

pub open spec fn shrink_nat_half(n: nat) -> nat {
    n / 2
}


pub open spec fn shrink_terminates_nat(n: nat) -> bool
    decreases n
{
    n == 0 || shrink_terminates_nat(shrink_nat_half(n))
}

} // verus!