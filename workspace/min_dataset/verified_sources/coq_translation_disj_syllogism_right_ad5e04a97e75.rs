use vstd::prelude::*;

verus! {

pub proof fn disj_syllogism_right(p: bool, q: bool)
    requires p || q, !q
    ensures p
{
}

} // verus!