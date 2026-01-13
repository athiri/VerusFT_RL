use vstd::prelude::*;

verus! {

pub proof fn lemma_contra(a: bool, b: bool)
    requires a ==> b,
        a ==> !b,
    ensures !a
{
    if a {
        assert(b);
        assert(!b);
    }
}

} // verus!