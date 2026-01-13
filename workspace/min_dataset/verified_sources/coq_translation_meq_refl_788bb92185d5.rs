use vstd::prelude::*;

verus! {

pub open spec fn mcount(m: Multiset, x: nat) -> nat {
    if m.dom().contains(x) { m[x] } else { 0 }
}


pub type Multiset = Map<nat, nat>;

pub open spec fn meq(m1: Multiset, m2: Multiset) -> bool {
    forall|x: nat| mcount(m1, x) == mcount(m2, x)
}


pub proof fn meq_refl(m: Multiset)
    ensures meq(m, m)
{
}

} // verus!