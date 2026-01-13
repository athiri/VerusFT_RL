use vstd::prelude::*;

verus! {

pub type NatList = Seq<nat>;

pub open spec fn app(xs: NatList, ys: NatList) -> NatList {
    xs.add(ys)
}

pub proof fn lemma_concat_associative(xs: NatList, ys: NatList, zs: NatList)
    ensures xs.add(ys).add(zs) =~= xs.add(ys.add(zs))
{}

pub proof fn ex3_app_assoc(xs: NatList, ys: NatList, zs: NatList)
    ensures app(app(xs, ys), zs) =~= app(xs, app(ys, zs))
{
    lemma_concat_associative(xs, ys, zs);
    assert(xs.add(ys).add(zs) =~= xs.add(ys.add(zs)));
}

} // verus!
