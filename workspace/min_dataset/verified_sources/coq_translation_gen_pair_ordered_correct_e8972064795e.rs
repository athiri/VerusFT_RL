use vstd::prelude::*;

verus! {

pub open spec fn gen_pair_ordered_outputs(bound: nat) -> Set<(nat, nat)> {
    Set::new(|p: (nat, nat)| p.0 < p.1 && p.1 <= bound)
}


pub proof fn gen_pair_ordered_correct(bound: nat, p: (nat, nat))
    requires gen_pair_ordered_outputs(bound).contains(p)
    ensures p.0 < p.1
{
}

} // verus!