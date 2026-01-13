use vstd::prelude::*;

verus! {

pub open spec fn mutation_score(killed: nat, total: nat) -> nat {
    if total == 0 {
        100  // No mutants = perfect score
    } else {
        (killed * 100) / total
    }
}


pub proof fn zero_score_when_none_killed(total: nat)
    requires total > 0
    ensures mutation_score(0, total) == 0
{
    assert((0 * 100) / total == 0);
}

} // verus!