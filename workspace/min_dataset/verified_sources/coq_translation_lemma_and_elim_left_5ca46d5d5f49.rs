use vstd::prelude::*;

verus! {

pub proof fn lemma_and_elim_left(a: bool, b: bool)
    requires a && b,
    ensures a
{
}

} // verus!