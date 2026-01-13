use vstd::prelude::*;

verus! {

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

} // verus!