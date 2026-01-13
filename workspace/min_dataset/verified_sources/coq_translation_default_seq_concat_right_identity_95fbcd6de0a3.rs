use vstd::prelude::*;

verus! {

pub open spec fn default_seq<A>() -> Seq<A> {
    Seq::empty()
}


pub proof fn default_seq_concat_right_identity<A>(xs: Seq<A>)
    ensures xs.add(default_seq::<A>()) =~= xs
{
    assert(xs.add(Seq::<A>::empty()) =~= xs);
}

} // verus!