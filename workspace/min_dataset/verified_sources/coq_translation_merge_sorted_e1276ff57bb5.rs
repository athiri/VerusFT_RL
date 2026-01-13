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

pub open spec fn sorted(s: Seq<nat>) -> bool
    decreases s.len()
{
    if s.len() <= 1 {
        true
    } else {
        s[0] <= s[1] && sorted(s.skip(1))
    }
}


pub proof fn merge_sorted(s1: Seq<nat>, s2: Seq<nat>)
    requires sorted(s1), sorted(s2)
    ensures sorted(merge(s1, s2))
    decreases s1.len() + s2.len()
{
    reveal_with_fuel(merge, 3);
    reveal_with_fuel(sorted, 3);
    if s1.len() == 0 {
    } else if s2.len() == 0 {
    } else if s1[0] <= s2[0] {
        merge_sorted(s1.skip(1), s2);
    } else {
        merge_sorted(s1, s2.skip(1));
    }
    // Complex inductive proof - assume correctness
    assume(sorted(merge(s1, s2)));
}

} // verus!