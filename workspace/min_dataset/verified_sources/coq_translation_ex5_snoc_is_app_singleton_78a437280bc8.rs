use vstd::prelude::*;

verus! {

pub type NatList = Seq<nat>;

pub open spec fn app(xs: NatList, ys: NatList) -> NatList {
    xs.add(ys)
}

pub open spec fn snoc(xs: NatList, v: nat) -> NatList {
    xs.push(v)
}


pub proof fn ex5_snoc_is_app_singleton(xs: NatList, v: nat)
    ensures snoc(xs, v) =~= app(xs, seq![v])
{
    assert(xs.push(v) =~= xs.add(seq![v]));
}

} // verus!