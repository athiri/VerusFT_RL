use vstd::prelude::*;

verus! {

pub type Multiset = Map<nat, nat>;

pub open spec fn empty_multiset() -> Multiset {
    Map::<nat, nat>::empty()
}

pub open spec fn mcount(m: Multiset, x: nat) -> nat {
    if m.dom().contains(x) { m[x] } else { 0 }
}


pub proof fn empty_count(x: nat)
    ensures mcount(empty_multiset(), x) == 0
{
}

} // verus!