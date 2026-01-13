use vstd::prelude::*;

verus! {

pub open spec fn seq_concat<A>(s1: Seq<A>, s2: Seq<A>) -> Seq<A> { s1 + s2 }


pub proof fn seq_concat_assoc<A>(s1: Seq<A>, s2: Seq<A>, s3: Seq<A>)
    ensures seq_concat(seq_concat(s1, s2), s3) =~= seq_concat(s1, seq_concat(s2, s3))
{
}

} // verus!