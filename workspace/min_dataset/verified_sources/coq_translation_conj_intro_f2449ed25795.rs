use vstd::prelude::*;

verus! {

pub proof fn conj_intro(p: bool, q: bool)
    requires p, q
    ensures p && q
{
}

} // verus!