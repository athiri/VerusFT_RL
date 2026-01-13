use vstd::prelude::*;

verus! {

pub open spec fn seq_fmap<A, B>(f: spec_fn(A) -> B, s: Seq<A>) -> Seq<B> {
    Seq::new(s.len(), |i: int| f(s[i]))
}


pub proof fn seq_fmap_length<A, B>(f: spec_fn(A) -> B, s: Seq<A>)
    ensures seq_fmap(f, s).len() == s.len()
{
}

} // verus!