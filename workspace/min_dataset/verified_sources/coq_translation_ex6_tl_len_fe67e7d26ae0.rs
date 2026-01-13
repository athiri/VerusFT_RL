use vstd::prelude::*;

verus! {

pub type NatList = Seq<nat>;

pub open spec fn tl(xs: NatList) -> NatList {
    if xs.len() == 0 {
        xs
    } else {
        xs.skip(1)
    }
}


pub proof fn ex6_tl_len(xs: NatList)
    requires xs.len() > 0,
    ensures tl(xs).len() + 1 == xs.len()
{
    assert(tl(xs) == xs.skip(1));
    assert(xs.skip(1).len() + 1 == xs.len());
}

} // verus!