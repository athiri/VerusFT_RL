use vstd::prelude::*;

verus! {

pub enum PalEv<A> {
    Empty,
    Single(A),
    Step(A, Box<PalEv<A>>),
}

impl<A> PalEv<A> {
    pub open spec fn seq(self) -> Seq<A>
        decreases self
    {
        match self {
            PalEv::Empty => Seq::empty(),
            PalEv::Single(a) => seq![a],
            PalEv::Step(a, mid) => seq![a].add((*mid).seq()).add(seq![a]),
        }
    }
}

} // verus!
