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


pub proof fn ex5_le_succ(n: nat) -> (p: LeEv)
    ensures p.lhs() == n && p.rhs() == n + 1
{
    let p = LeEv::Step(Box::new(LeEv::Refl(n)));
    reveal_with_fuel(LeEv::lhs, 3);
    reveal_with_fuel(LeEv::rhs, 3);
    assert(p.lhs() == n);
    assert(p.rhs() == n + 1);
    p
}

} // verus!