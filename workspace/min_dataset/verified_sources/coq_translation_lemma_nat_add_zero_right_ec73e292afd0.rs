use vstd::prelude::*;

verus! {

pub proof fn lemma_nat_add_zero_right(n: nat)
    ensures n + 0 == n
{
}

} // verus!