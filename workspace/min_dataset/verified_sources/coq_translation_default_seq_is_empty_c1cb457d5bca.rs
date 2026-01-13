use vstd::prelude::*;

verus! {

pub open spec fn default_seq<A>() -> Seq<A> {
    Seq::empty()
}


pub proof fn default_seq_is_empty<A>()
    ensures default_seq::<A>().len() == 0
{
    assert(Seq::<A>::empty().len() == 0);
}

} // verus!