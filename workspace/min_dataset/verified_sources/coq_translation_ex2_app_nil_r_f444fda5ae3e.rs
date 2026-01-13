use vstd::prelude::*;

verus! {

pub type NatList = Seq<nat>;

pub open spec fn app(xs: NatList, ys: NatList) -> NatList {
    xs.add(ys)
}


pub proof fn ex2_app_nil_r(xs: NatList)
    ensures app(xs, Seq::empty()) =~= xs
{
    assert(xs.add(Seq::empty()) =~= xs);
}

} // verus!