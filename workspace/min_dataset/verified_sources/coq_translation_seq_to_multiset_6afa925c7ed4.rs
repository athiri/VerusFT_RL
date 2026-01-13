use vstd::prelude::*;

verus! {

pub open spec fn mcount(m: Multiset, x: nat) -> nat {
    if m.dom().contains(x) { m[x] } else { 0 }
}


pub type Multiset = Map<nat, nat>;

pub open spec fn madd(m: Multiset, x: nat) -> Multiset {
    m.insert(x, mcount(m, x) + 1)
}

pub open spec fn empty_multiset() -> Multiset {
    Map::<nat, nat>::empty()
}


pub open spec fn seq_to_multiset(s: Seq<nat>) -> Multiset
    decreases s.len()
{
    if s.len() == 0 {
        empty_multiset()
    } else {
        madd(seq_to_multiset(s.skip(1)), s[0])
    }
}

} // verus!