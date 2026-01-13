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

    pub open spec fn rhs(self) -> nat
        decreases self
    {
        match self {
            LeEv::Refl(n) => n,
            LeEv::Step(p) => (*p).rhs() + 1,
        }
    }
}


pub proof fn ex4_le_refl(n: nat) -> (p: LeEv)
    ensures p.lhs() == n && p.rhs() == n
{
    LeEv::Refl(n)
}

} // verus!