use vstd::prelude::*;

verus! {

pub open spec fn is_even(n: nat) -> bool {
    n % 2 == 0
}

pub struct EvenEv {
    pub n: nat,
}


pub proof fn even_ev_new(n: nat) -> (ev: EvenEv)
    requires is_even(n)
    ensures ev.n == n,
        is_even(ev.n)
{
    EvenEv { n }
}

} // verus!