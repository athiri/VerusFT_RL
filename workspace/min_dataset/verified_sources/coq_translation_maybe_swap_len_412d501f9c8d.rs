use vstd::prelude::*;

verus! {

pub open spec fn swap_at<T>(s: Seq<T>, i: nat, j: nat) -> Seq<T>
    recommends i < s.len(), j < s.len()
{
    s.update(i as int, s[j as int]).update(j as int, s[i as int])
}


pub open spec fn maybe_swap(s: Seq<nat>, i: nat, j: nat) -> Seq<nat>
    recommends i < s.len(), j < s.len()
{
    if s[i as int] > s[j as int] {
        swap_at(s, i, j)
    } else {
        s
    }
}


pub proof fn maybe_swap_len(s: Seq<nat>, i: nat, j: nat)
    requires i < s.len(), j < s.len()
    ensures maybe_swap(s, i, j).len() == s.len()
{
}

} // verus!