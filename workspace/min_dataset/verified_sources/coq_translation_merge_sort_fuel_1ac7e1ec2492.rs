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

pub open spec fn split_second(s: Seq<nat>) -> Seq<nat> {
    s.skip((s.len() / 2) as int)
}

pub open spec fn split_first(s: Seq<nat>) -> Seq<nat> {
    s.take((s.len() / 2) as int)
}


pub open spec fn merge_sort_fuel(s: Seq<nat>, fuel: nat) -> Seq<nat>
    decreases fuel
{
    if fuel == 0 || s.len() <= 1 {
        s
    } else {
        let s1 = split_first(s);
        let s2 = split_second(s);
        let sorted1 = merge_sort_fuel(s1, (fuel - 1) as nat);
        let sorted2 = merge_sort_fuel(s2, (fuel - 1) as nat);
        merge(sorted1, sorted2)
    }
}

} // verus!