use vstd::prelude::*;

verus! {

pub proof fn disj_syllogism_left(p: bool, q: bool)
    requires p || q, !p
    ensures q
{
}

} // verus!