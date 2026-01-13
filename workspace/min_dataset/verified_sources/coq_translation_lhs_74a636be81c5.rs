use vstd::prelude::*;

verus! {

pub enum LeEv {
    Refl(nat),
    Step(Box<LeEv>),
}
impl LeEv {
    pub open spec fn lhs(self) -> nat
        decreases self
    {
        match self {
            LeEv::Refl(n) => n,
            LeEv::Step(p) => (*p).lhs(),
        }
}




    }

} // verus!