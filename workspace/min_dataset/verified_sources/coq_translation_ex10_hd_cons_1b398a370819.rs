use vstd::prelude::*;

verus! {

pub type NatList = Seq<nat>;

pub open spec fn hd(default: nat, xs: NatList) -> nat {
    if xs.len() == 0 {
        default
    } else {
        xs[0]
    }
}


pub proof fn ex10_hd_cons(d: nat, x: nat, xs: NatList)
    ensures hd(d, seq![x].add(xs)) == x
{
    assert(seq![x].add(xs).len() > 0);
    assert(hd(d, seq![x].add(xs)) == (seq![x].add(xs))[0]);
    assert((seq![x].add(xs))[0] == x);
}

} // verus!