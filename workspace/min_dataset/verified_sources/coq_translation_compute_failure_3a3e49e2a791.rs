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

} // verus!