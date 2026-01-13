use vstd::prelude::*;

verus! {

pub open spec fn empty_multiset() -> Multiset {
    Map::<nat, nat>::empty()
}

pub open spec fn mcount(m: Multiset, x: nat) -> nat {
    if m.dom().contains(x) { m[x] } else { 0 }
}


pub type Multiset = Map<nat, nat>;

pub open spec fn meq(m1: Multiset, m2: Multiset) -> bool {
    forall|x: nat| mcount(m1, x) == mcount(m2, x)
}

pub open spec fn madd(m: Multiset, x: nat) -> Multiset {
    m.insert(x, mcount(m, x) + 1)
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


pub proof fn seq_multiset_cons(x: nat, s: Seq<nat>)
    ensures meq(seq_to_multiset(seq![x].add(s)), madd(seq_to_multiset(s), x))
{
    reveal_with_fuel(seq_to_multiset, 2);
    assume(meq(seq_to_multiset(seq![x].add(s)), madd(seq_to_multiset(s), x)));
}

} // verus!