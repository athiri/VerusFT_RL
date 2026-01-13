use vstd::prelude::*;

verus! {

pub open spec fn shrink_nat_half(n: nat) -> nat {
    n / 2
}


pub proof fn verify_shrink_nat_smaller(n: nat)
    requires n > 0
    ensures shrink_nat_half(n) < n
{
}

} // verus!