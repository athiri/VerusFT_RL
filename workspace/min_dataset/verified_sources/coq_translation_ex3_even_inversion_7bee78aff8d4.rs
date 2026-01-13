use vstd::prelude::*;

verus! {
#[verifier::accept_recursive_types]
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


pub proof fn ex3_even_inversion(e: EvenEv)
    requires e.n() > 0,
    ensures match e { EvenEv::EvSS(_) => true, _ => false }
{}

} // verus!