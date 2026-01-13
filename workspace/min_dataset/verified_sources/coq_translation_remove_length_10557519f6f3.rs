use vstd::prelude::*;

verus! {

pub open spec fn remove_at(s: Seq<nat>, i: int) -> Seq<nat>
    recommends 0 <= i < s.len()
{
    s.take(i).add(s.skip(i + 1))
}


pub proof fn remove_length(s: Seq<nat>, i: int)
    requires 0 <= i < s.len()
    ensures remove_at(s, i).len() == s.len() - 1
{
}

} // verus!