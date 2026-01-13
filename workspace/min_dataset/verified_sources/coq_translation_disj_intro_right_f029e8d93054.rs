use vstd::prelude::*;

verus! {

pub proof fn disj_intro_right(p: bool, q: bool)
    requires q
    ensures p || q
{
}

} // verus!