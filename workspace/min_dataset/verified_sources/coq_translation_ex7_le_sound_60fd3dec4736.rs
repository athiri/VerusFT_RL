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


pub proof fn ex7_le_sound(p: LeEv)
    ensures p.lhs() <= p.rhs()
    decreases p
{
    match p {
        LeEv::Refl(_) => {}
        LeEv::Step(p1) => {
            ex7_le_sound(*p1);
            assert(p.lhs() == p1.lhs());
            assert(p.rhs() == p1.rhs() + 1);
        }
    }
}

} // verus!