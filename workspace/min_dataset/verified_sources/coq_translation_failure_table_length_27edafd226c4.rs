use vstd::prelude::*;

verus! {

pub open spec fn compute_failure(pattern: Seq<nat>, i: nat, j: nat) -> Seq<nat>
    decreases pattern.len() - i
{
    if i >= pattern.len() { Seq::empty() }
    else {
        let new_j = if j > 0 && pattern[i as int] == pattern[j as int] { j + 1 } else { 0 };
        seq![new_j] + compute_failure(pattern, i + 1, new_j)
    }
}


pub open spec fn failure_table(pattern: Seq<nat>) -> Seq<nat> {
    seq![0nat] + compute_failure(pattern, 1, 0)
}


pub proof fn failure_table_length(pattern: Seq<nat>)
    ensures failure_table(pattern).len() >= 1
{
}

} // verus!