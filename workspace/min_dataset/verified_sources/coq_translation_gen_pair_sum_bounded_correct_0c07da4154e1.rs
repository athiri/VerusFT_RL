use vstd::prelude::*;

verus! {

pub open spec fn gen_pair_sum_bounded_outputs(bound: nat) -> Set<(nat, nat)> {
    Set::new(|p: (nat, nat)| p.0 + p.1 <= bound)
}


pub proof fn gen_pair_sum_bounded_correct(bound: nat, p: (nat, nat))
    requires gen_pair_sum_bounded_outputs(bound).contains(p)
    ensures p.0 + p.1 <= bound
{
}

} // verus!