use vstd::prelude::*;

verus! {

pub open spec fn split_second(s: Seq<nat>) -> Seq<nat> {
    s.skip((s.len() / 2) as int)
}

pub open spec fn split_first(s: Seq<nat>) -> Seq<nat> {
    s.take((s.len() / 2) as int)
}


pub proof fn split_length(s: Seq<nat>)
    ensures split_first(s).len() + split_second(s).len() == s.len()
{
}

} // verus!