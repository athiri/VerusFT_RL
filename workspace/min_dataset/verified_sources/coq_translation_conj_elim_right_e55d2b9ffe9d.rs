use vstd::prelude::*;

verus! {

pub proof fn conj_elim_right(p: bool, q: bool)
    requires p && q
    ensures q
{
}

} // verus!