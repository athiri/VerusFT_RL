use vstd::prelude::*;

verus! {

pub open spec fn swap_at<T>(s: Seq<T>, i: nat, j: nat) -> Seq<T>
    recommends i < s.len(), j < s.len()
{
    s.update(i as int, s[j as int]).update(j as int, s[i as int])
}


pub proof fn swap_self_id<T>(s: Seq<T>, i: nat)
    requires i < s.len()
    ensures swap_at(s, i, i) =~= s
{
}

} // verus!