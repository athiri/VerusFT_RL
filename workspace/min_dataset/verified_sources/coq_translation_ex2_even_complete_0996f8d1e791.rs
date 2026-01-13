use vstd::prelude::*;

verus! {

pub open spec fn is_even(n: nat) -> bool
    decreases n
{
    if n == 0 {
        true
    } else if n == 1 {
        false
    } else {
        is_even((n - 2) as nat)
    }
}

pub enum EvenEv {
    Ev0,
    EvSS(Box<EvenEv>),
}

impl EvenEv {
    pub open spec fn n(self) -> nat
        decreases self
    {
        match self {
            EvenEv::Ev0 => 0,
            EvenEv::EvSS(e) => (*e).n() + 2,
        }
    }
}


pub proof fn ex2_even_complete(n: nat) -> (e: EvenEv)
    requires is_even(n),
    ensures e.n() == n,
    decreases n
{
    if n == 0 {
        EvenEv::Ev0
    } else {
        // For nat, if is_even(n) and n != 0, then n >= 2.
        assert(n != 1);
        assert(n >= 2);
        let n2 = (n - 2) as nat;
        let e2 = ex2_even_complete(n2);
        EvenEv::EvSS(Box::new(e2))
    }
}

} // verus!