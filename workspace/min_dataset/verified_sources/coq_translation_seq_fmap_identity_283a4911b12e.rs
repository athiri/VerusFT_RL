use vstd::prelude::*;

verus! {

pub open spec fn seq_fmap<A, B>(f: spec_fn(A) -> B, s: Seq<A>) -> Seq<B> {
    Seq::new(s.len(), |i: int| f(s[i]))
}


pub proof fn seq_fmap_identity(s: Seq<nat>)
    ensures seq_fmap(|x: nat| x, s) =~= s
{
    assert forall|i: int| 0 <= i < s.len() implies seq_fmap(|x: nat| x, s)[i] == s[i] by {}
}

} // verus!