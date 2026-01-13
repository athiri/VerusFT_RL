use vstd::prelude::*;

verus! {

pub proof fn or_intro_left(p: bool, q: bool)
    requires p
    ensures p || q
{
}

} // verus!