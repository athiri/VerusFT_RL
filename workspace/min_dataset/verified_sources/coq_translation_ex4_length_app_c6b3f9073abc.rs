use vstd::prelude::*;

verus! {

pub type NatList = Seq<nat>;

pub open spec fn length(xs: NatList) -> nat {
    xs.len()
}

pub open spec fn app(xs: NatList, ys: NatList) -> NatList {
    xs.add(ys)
}


pub proof fn ex4_length_app(xs: NatList, ys: NatList)
    ensures length(app(xs, ys)) == length(xs) + length(ys)
{
    assert(xs.add(ys).len() == xs.len() + ys.len());
}

} // verus!