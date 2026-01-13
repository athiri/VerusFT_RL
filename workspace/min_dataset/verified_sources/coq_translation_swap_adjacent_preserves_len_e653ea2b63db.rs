use vstd::prelude::*;

verus! {

pub open spec fn swap_at<T>(s: Seq<T>, i: nat, j: nat) -> Seq<T>
    recommends i < s.len(), j < s.len()
{
    s.update(i as int, s[j as int]).update(j as int, s[i as int])
}


pub open spec fn swap_adjacent(s: Seq<nat>, i: nat) -> Seq<nat>
    recommends i + 1 < s.len()
{
    swap_at(s, i, i + 1)
}


pub proof fn swap_adjacent_preserves_len(s: Seq<nat>, i: nat)
    requires i + 1 < s.len()
    ensures swap_adjacent(s, i).len() == s.len()
{
}

} // verus!