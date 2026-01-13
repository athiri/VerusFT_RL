use vstd::prelude::*;

verus! {

pub open spec fn mcount(m: Multiset, x: nat) -> nat {
    if m.dom().contains(x) { m[x] } else { 0 }
}


pub type Multiset = Map<nat, nat>;

pub open spec fn meq(m1: Multiset, m2: Multiset) -> bool {
    forall|x: nat| mcount(m1, x) == mcount(m2, x)
}


pub proof fn meq_sym(m1: Multiset, m2: Multiset)
    requires meq(m1, m2)
    ensures meq(m2, m1)
{
}

} // verus!