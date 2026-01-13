use vstd::prelude::*;

verus! {

pub open spec fn linear_search(s: Seq<nat>, target: nat, i: nat) -> Option<nat>
    decreases s.len() - i
{
    if i >= s.len() {
        None
    } else if s[i as int] == target {
        Some(i)
    } else {
        linear_search(s, target, (i + 1) as nat)
    }
}

pub proof fn search_finds_target(s: Seq<nat>, target: nat, i: nat)
    ensures match linear_search(s, target, i) {
        Some(idx) => i <= idx < s.len() && s[idx as int] == target,
        None => forall|j: nat| i <= j < s.len() ==> s[j as int] != target,
    }
    decreases s.len() - i
{
    if i < s.len() && s[i as int] != target {
        search_finds_target(s, target, (i + 1) as nat);
    }
}

} // verus!
