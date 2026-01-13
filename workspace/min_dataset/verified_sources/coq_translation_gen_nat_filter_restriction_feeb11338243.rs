use vstd::prelude::*;

verus! {

pub open spec fn gen_nat_filter(outputs: Set<nat>, p: spec_fn(nat) -> bool) -> Set<nat> {
    Set::new(|n: nat| outputs.contains(n) && p(n))
}


pub proof fn gen_nat_filter_restriction(outputs: Set<nat>, p: spec_fn(nat) -> bool, n: nat)
    requires gen_nat_filter(outputs, p).contains(n)
    ensures outputs.contains(n) && p(n)
{
}

} // verus!