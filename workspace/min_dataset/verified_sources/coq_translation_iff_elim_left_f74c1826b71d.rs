use vstd::prelude::*;

verus! {

pub proof fn iff_elim_left(p: bool, q: bool)
    requires p <==> q, p
    ensures q
{
}

} // verus!