use vstd::prelude::*;

verus! {

pub struct Pair<A, B> {
    pub fst: A,
    pub snd: B,
}


pub open spec fn shrink_pair_snd<A, B>(p: Pair<A, B>, shrunk_bs: Seq<B>) -> Seq<Pair<A, B>>
    where A: std::marker::Copy, B: std::marker::Copy
    decreases shrunk_bs.len()
{
    if shrunk_bs.len() == 0 {
        seq![]
    } else {
        seq![Pair { fst: p.fst, snd: shrunk_bs[0] }] +
            shrink_pair_snd(p, shrunk_bs.drop_first())
    }
}

} // verus!