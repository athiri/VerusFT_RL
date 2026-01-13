use vstd::prelude::*;

verus! {

pub open spec fn is_mul4(n: nat) -> bool {
    n % 4 == 0
}

pub struct Mul4Ev {
    pub n: nat,
}


pub proof fn mul4_ev_new(n: nat) -> (ev: Mul4Ev)
    requires is_mul4(n)
    ensures ev.n == n,
        is_mul4(ev.n)
{
    Mul4Ev { n }
}

} // verus!