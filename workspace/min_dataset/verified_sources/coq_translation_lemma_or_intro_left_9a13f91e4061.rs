use vstd::prelude::*;

verus! {

pub proof fn lemma_or_intro_left(a: bool, b: bool)
    requires a,
    ensures a || b
{
}

} // verus!