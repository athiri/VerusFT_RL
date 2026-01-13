use vstd::prelude::*;

verus! {

pub proof fn lemma_and_elim_right(a: bool, b: bool)
    requires a && b,
    ensures b
{
}

} // verus!