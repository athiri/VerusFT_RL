use vstd::prelude::*;

verus! {

pub proof fn neg_elim_contradiction(p: bool, q: bool)
    requires p, !p
    ensures q
{
}

} // verus!