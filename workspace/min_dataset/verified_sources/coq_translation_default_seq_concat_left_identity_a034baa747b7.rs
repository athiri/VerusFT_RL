use vstd::prelude::*;

verus! {

pub open spec fn default_seq<A>() -> Seq<A> {
    Seq::empty()
}


pub proof fn default_seq_concat_left_identity<A>(xs: Seq<A>)
    ensures default_seq::<A>().add(xs) =~= xs
{
    assert(Seq::<A>::empty().add(xs) =~= xs);
}

} // verus!