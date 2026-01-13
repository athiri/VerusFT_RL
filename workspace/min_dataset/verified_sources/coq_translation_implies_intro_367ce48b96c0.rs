use vstd::prelude::*;

verus! {

pub proof fn implies_intro(p: bool, q: bool)
    requires p ==> q
    ensures p ==> q
{
}

} // verus!