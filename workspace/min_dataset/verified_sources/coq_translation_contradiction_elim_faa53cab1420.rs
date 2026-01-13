use vstd::prelude::*;

verus! {

pub proof fn contradiction_elim(p: bool)
    requires false
    ensures p
{
}

} // verus!