use vstd::prelude::*;

verus! {

pub proof fn and_elim_right(p: bool, q: bool)
    requires p && q
    ensures q
{
}

} // verus!