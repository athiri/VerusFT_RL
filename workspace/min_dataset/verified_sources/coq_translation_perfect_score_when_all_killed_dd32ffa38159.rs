use vstd::prelude::*;

verus! {

pub open spec fn mutation_score(killed: nat, total: nat) -> nat {
    if total == 0 {
        100  // No mutants = perfect score
    } else {
        (killed * 100) / total
    }
}


pub proof fn perfect_score_when_all_killed(killed: nat, total: nat)
    requires killed == total, total > 0
    ensures mutation_score(killed, total) == 100
{
    assert((killed * 100) / killed == 100) by(nonlinear_arith)
        requires killed > 0;
}

} // verus!