use vstd::prelude::*;

verus! {

pub struct Pair<A, B> {
    pub fst: A,
    pub snd: B,
}


pub open spec fn shrink_pair_fst<A, B>(p: Pair<A, B>, shrunk_as: Seq<A>) -> Seq<Pair<A, B>>
    where A: std::marker::Copy, B: std::marker::Copy
    decreases shrunk_as.len()
{
    if shrunk_as.len() == 0 {
        seq![]
    } else {
        seq![Pair { fst: shrunk_as[0], snd: p.snd }] +
            shrink_pair_fst(p, shrunk_as.drop_first())
    }
}

} // verus!