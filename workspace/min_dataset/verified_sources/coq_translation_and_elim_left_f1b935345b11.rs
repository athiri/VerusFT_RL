use vstd::prelude::*;

verus! {

pub proof fn and_elim_left(p: bool, q: bool)
    requires p && q
    ensures p
{
}

} // verus!