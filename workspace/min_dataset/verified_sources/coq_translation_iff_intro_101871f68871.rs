use vstd::prelude::*;

verus! {

pub proof fn iff_intro(p: bool, q: bool)
    requires p ==> q, q ==> p
    ensures p <==> q
{
}

} // verus!