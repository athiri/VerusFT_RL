use vstd::prelude::*;

verus! {

pub open spec fn sum_to(n: nat) -> nat
    decreases n
{
    if n == 0 { 0 } else { sum_to((n - 1) as nat) + ((n - 1) as nat) }
}


pub proof fn lemma_sum_to_succ(n: nat)
    ensures sum_to(n + 1) == sum_to(n) + n
{
    // By unfolding sum_to(n+1): it reduces to sum_to(n) + n.
}

} // verus!