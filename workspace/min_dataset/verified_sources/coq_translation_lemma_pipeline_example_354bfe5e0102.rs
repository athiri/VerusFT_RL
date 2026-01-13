use vstd::prelude::*;

verus! {

pub proof fn lemma_pipeline_example(a: bool, b: bool, c: bool)
    requires a,
        a ==> b,
        b ==> c,
    ensures c
{
    // Think of `a` as a proof term for proposition `a`.
    assert(b);
    assert(c);
}

} // verus!