use vstd::prelude::*;

verus! {

pub proof fn lemma_add_0_l(n: nat)
    ensures add(0, n) == n
{
}

} // verus!