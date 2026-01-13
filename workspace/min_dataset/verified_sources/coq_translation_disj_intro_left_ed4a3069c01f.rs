use vstd::prelude::*;

verus! {

pub proof fn disj_intro_left(p: bool, q: bool)
    requires p
    ensures p || q
{
}

} // verus!