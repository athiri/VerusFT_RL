use vstd::prelude::*;

verus! {

pub open spec fn seq_concat<A>(s1: Seq<A>, s2: Seq<A>) -> Seq<A> { s1 + s2 }

pub open spec fn seq_concat_identity<A>() -> Seq<A> { Seq::empty() }


pub proof fn seq_concat_right_identity<A>(s: Seq<A>)
    ensures seq_concat(s, seq_concat_identity()) =~= s
{
}

} // verus!