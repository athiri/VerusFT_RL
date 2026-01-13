use vstd::prelude::*;

verus! {

pub proof fn iff_elim_right(p: bool, q: bool)
    requires p <==> q, q
    ensures p
{
}

} // verus!