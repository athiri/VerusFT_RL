use vstd::prelude::*;

verus! {

pub proof fn lemma_add_succ_l(x: nat, m: nat)
    ensures add(x + 1, m) == add(x, m) + 1
{
    // `x + 1` is always nonzero for nat, so unfolding is safe.
    assert(add(x + 1, m) == add(x, m) + 1);
}

} // verus!