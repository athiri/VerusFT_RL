use vstd::prelude::*;

verus! {

pub open spec fn maybe_swap(s: Seq<nat>, i: nat, j: nat) -> Seq<nat>
    recommends i < s.len(), j < s.len()
{
    if s[i as int] > s[j as int] {
        swap_at(s, i, j)
    } else {
        s
    }
}

pub open spec fn swap_at<T>(s: Seq<T>, i: nat, j: nat) -> Seq<T>
    recommends i < s.len(), j < s.len()
{
    s.update(i as int, s[j as int]).update(j as int, s[i as int])
}


pub proof fn maybe_swap_ordered(s: Seq<nat>, i: nat, j: nat)
    requires i < s.len(), j < s.len()
    ensures maybe_swap(s, i, j)[i as int] <= maybe_swap(s, i, j)[j as int]
{
    if s[i as int] > s[j as int] {
        assert(swap_at(s, i, j)[i as int] == s[j as int]);
        assert(swap_at(s, i, j)[j as int] == s[i as int]);
    }
}

} // verus!