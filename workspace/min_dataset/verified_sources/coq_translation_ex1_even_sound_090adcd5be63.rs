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


pub proof fn ex1_even_sound(e: EvenEv)
    ensures is_even(e.n())
    decreases e
{
    match e {
        EvenEv::Ev0 => {
            assert(is_even(0));
        }
        EvenEv::EvSS(e1) => {
            ex1_even_sound(*e1);
            assert(is_even(e.n()) == is_even((e1.n()) as nat));
        }
    }
}

} // verus!