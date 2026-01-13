use vstd::prelude::*;

verus! {

pub proof fn lemma_and_intro(a: bool, b: bool)
    requires a,
        b,
    ensures a && b
{
}

} // verus!