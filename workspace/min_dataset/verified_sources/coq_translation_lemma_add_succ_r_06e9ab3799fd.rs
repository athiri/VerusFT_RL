use vstd::prelude::*;

verus! {

pub proof fn lemma_add_succ_r(n: nat, m: nat)
    ensures add(n, m + 1) == add(n, m) + 1
    decreases n
{
    if n == 0 {
    } else {
        lemma_add_succ_r((n - 1) as nat, m);
    }
}

} // verus!