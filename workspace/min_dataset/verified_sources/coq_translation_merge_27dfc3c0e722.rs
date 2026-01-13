use vstd::prelude::*;

verus! {

pub open spec fn merge(s1: Seq<nat>, s2: Seq<nat>) -> Seq<nat>
    decreases s1.len() + s2.len()
{
    if s1.len() == 0 {
        s2
    } else if s2.len() == 0 {
        s1
    } else if s1[0] <= s2[0] {
        seq![s1[0]].add(merge(s1.skip(1), s2))
    } else {
        seq![s2[0]].add(merge(s1, s2.skip(1)))
    }
}

} // verus!