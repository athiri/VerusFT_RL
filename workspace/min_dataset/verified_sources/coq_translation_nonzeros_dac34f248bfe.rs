use vstd::prelude::*;

verus! {

pub type NatList = Seq<nat>;


pub open spec fn nonzeros(xs: NatList) -> NatList
    decreases xs.len()
{
    if xs.len() == 0 {
        Seq::empty()
    } else {
        let x = xs[0];
        let rest = nonzeros(xs.skip(1));
        if x == 0 {
            rest
        } else {
            seq![x].add(rest)
        }
    }
}

} // verus!