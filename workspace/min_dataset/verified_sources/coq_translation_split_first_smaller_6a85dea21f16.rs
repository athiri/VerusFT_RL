use vstd::prelude::*;

verus! {

pub open spec fn split_first(s: Seq<nat>) -> Seq<nat> {
    s.take((s.len() / 2) as int)
}


pub proof fn split_first_smaller(s: Seq<nat>)
    requires s.len() > 1
    ensures split_first(s).len() < s.len()
{
}

} // verus!