use vstd::prelude::*;

verus! {

pub proof fn and_intro(p: bool, q: bool)
    requires p, q
    ensures p && q
{
}

} // verus!