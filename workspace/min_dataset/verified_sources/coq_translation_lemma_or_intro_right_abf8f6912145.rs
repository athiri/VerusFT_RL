use vstd::prelude::*;

verus! {

pub proof fn lemma_or_intro_right(a: bool, b: bool)
    requires b,
    ensures a || b
{
}

} // verus!