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


pub open spec fn is_permutation(s1: Seq<nat>, s2: Seq<nat>) -> bool {
    s1.len() == s2.len() &&
    forall|x: nat| count_occurrences(s1, x) == count_occurrences(s2, x)
}


pub proof fn perm_nil()
    ensures is_permutation(Seq::<nat>::empty(), Seq::<nat>::empty())
{
}

} // verus!