use vstd::prelude::*;

verus! {

pub open spec fn swap_at<T>(s: Seq<T>, i: nat, j: nat) -> Seq<T>
    recommends i < s.len(), j < s.len()
{
    s.update(i as int, s[j as int]).update(j as int, s[i as int])
}


pub proof fn swap_preserves_len<T>(s: Seq<T>, i: nat, j: nat)
    requires i < s.len(), j < s.len()
    ensures swap_at(s, i, j).len() == s.len()
{
}

} // verus!