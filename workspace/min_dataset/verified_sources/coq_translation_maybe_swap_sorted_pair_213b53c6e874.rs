use vstd::prelude::*;

verus! {

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


pub proof fn maybe_swap_sorted_pair(s: Seq<nat>)
    requires s.len() >= 2
    ensures maybe_swap(s)[0] <= maybe_swap(s)[1]
{
}

} // verus!