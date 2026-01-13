use vstd::prelude::*;

verus! {

pub open spec fn count(s: Seq<nat>, v: nat) -> nat
    decreases s.len()
{
    if s.len() == 0 {
        0
    } else if s[0] == v {
        1 + count(s.skip(1), v)
    } else {
        count(s.skip(1), v)
    }
}


pub open spec fn same_counts(s1: Seq<nat>, s2: Seq<nat>) -> bool {
    forall|v: nat| count(s1, v) == count(s2, v)
}


pub proof fn same_counts_trans(s1: Seq<nat>, s2: Seq<nat>, s3: Seq<nat>)
    requires same_counts(s1, s2), same_counts(s2, s3)
    ensures same_counts(s1, s3)
{
}

} // verus!