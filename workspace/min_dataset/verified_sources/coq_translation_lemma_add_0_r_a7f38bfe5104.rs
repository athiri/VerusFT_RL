use vstd::prelude::*;

verus! {

pub proof fn lemma_add_0_r(n: nat)
    ensures add(n, 0) == n
    decreases n
{
    if n == 0 {
    } else {
        lemma_add_0_r((n - 1) as nat);
    }
}

} // verus!