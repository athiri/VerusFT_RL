use vstd::prelude::*;

verus! {

pub open spec fn shrink_nat_dec(n: nat) -> nat {
    if n > 0 { (n - 1) as nat } else { 0 }
}


pub proof fn verify_shrink_nat_dec_smaller(n: nat)
    requires n > 0
    ensures shrink_nat_dec(n) < n
{
}

} // verus!