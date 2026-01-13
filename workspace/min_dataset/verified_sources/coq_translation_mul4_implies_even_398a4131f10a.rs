use vstd::prelude::*;

verus! {

pub open spec fn is_even(n: nat) -> bool {
    n % 2 == 0
}

pub struct EvenEv {
    pub n: nat,
}

pub open spec fn is_mul4(n: nat) -> bool {
    n % 4 == 0
}

pub struct Mul4Ev {
    pub n: nat,
}

pub proof fn even_ev_new(n: nat) -> (ev: EvenEv)
    requires is_even(n)
    ensures ev.n == n,
        is_even(ev.n)
{
    EvenEv { n }
}


pub proof fn mul4_implies_even(ev4: Mul4Ev) -> (ev2: EvenEv)
    requires is_mul4(ev4.n)
    ensures ev2.n == ev4.n,
        is_even(ev2.n)
{
    // Arithmetic fact: 4|n => 2|n
    assert(is_even(ev4.n)) by {
        // If n % 4 == 0 then n % 2 == 0
        // Verus can discharge this via nonlinear arithmetic on mod.
    }
    even_ev_new(ev4.n)
}

} // verus!