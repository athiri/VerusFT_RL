use vstd::prelude::*;

verus! {

pub open spec fn split_second(s: Seq<nat>) -> Seq<nat> {
    s.skip((s.len() / 2) as int)
}


pub proof fn split_second_smaller(s: Seq<nat>)
    requires s.len() > 1
    ensures split_second(s).len() < s.len()
{
}

} // verus!