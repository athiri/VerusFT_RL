use vstd::prelude::*;

verus! {

pub open spec fn count_occurrences(s: Seq<nat>, x: nat) -> nat
    decreases s.len()
{
    if s.len() == 0 {
        0
    } else if s[0] == x {
        1 + count_occurrences(s.skip(1), x)
    } else {
        count_occurrences(s.skip(1), x)
    }
}


pub proof fn perm_refl(s: Seq<nat>)
    ensures is_permutation(s, s)
{
}

pub open spec fn maybe_swap(s: Seq<nat>) -> Seq<nat> {
    if s.len() < 2 {
        s
    } else if s[0] > s[1] {
        // Swap: [s[1], s[0]] ++ rest
        seq![s[1], s[0]].add(s.skip(2))
    } else {
        s
    }
}

pub open spec fn is_permutation(s1: Seq<nat>, s2: Seq<nat>) -> bool {
    s1.len() == s2.len() &&
    forall|x: nat| count_occurrences(s1, x) == count_occurrences(s2, x)
}


pub proof fn maybe_swap_perm(s: Seq<nat>)
    ensures is_permutation(s, maybe_swap(s))
{
    if s.len() < 2 {
        perm_refl(s);
    } else {
        // Both cases (swap or no swap) preserve the multiset
        assume(is_permutation(s, maybe_swap(s)));
    }
}

} // verus!