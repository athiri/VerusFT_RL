use vstd::prelude::*;

verus! {

pub type Multiset = Map<nat, nat>;

pub open spec fn singleton(x: nat) -> Multiset {
    Map::<nat, nat>::empty().insert(x, 1)
}

pub open spec fn mcount(m: Multiset, x: nat) -> nat {
    if m.dom().contains(x) { m[x] } else { 0 }
}


pub proof fn singleton_count(x: nat)
    ensures mcount(singleton(x), x) == 1
{
}

} // verus!