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


pub proof fn ex6_le_trans(p: LeEv, q: LeEv) -> (r: LeEv)
    requires p.rhs() == q.lhs(),
    ensures r.lhs() == p.lhs() && r.rhs() == q.rhs(),
    decreases q
{
    match q {
        LeEv::Refl(_) => {
            // q.rhs == q.lhs, so p already has rhs == q.rhs.
            p
        }
        LeEv::Step(q1) => {
            let mid = ex6_le_trans(p, *q1);
            LeEv::Step(Box::new(mid))
        }
    }
}

} // verus!