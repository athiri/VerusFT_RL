use vstd::prelude::*;

verus! {

pub proof fn lemma_imp_trans(a: bool, b: bool, c: bool)
    requires a ==> b,
        b ==> c,
    ensures a ==> c
{
    // Proof object: a function from a proof of a to a proof of c.
    assert(a ==> c) by {
        if a {
            assert(b);
            assert(c);
        }
    }
}

} // verus!