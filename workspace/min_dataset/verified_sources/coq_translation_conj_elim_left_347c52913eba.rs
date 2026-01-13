use vstd::prelude::*;

verus! {

pub proof fn conj_elim_left(p: bool, q: bool)
    requires p && q
    ensures p
{
}

} // verus!