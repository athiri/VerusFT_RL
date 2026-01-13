use vstd::prelude::*;

verus! {

pub open spec fn shrink_seq_nat(xs: Seq<nat>) -> Seq<Seq<nat>> {
    if xs.len() == 0 {
        Seq::empty()
    } else if xs.len() == 1 {
        seq![Seq::empty()]
    } else {
        Seq::new(xs.len(), |i: int| xs.remove(i))
    }
}


pub proof fn shrink_seq_shorter(xs: Seq<nat>, i: int)
    requires xs.len() > 1,
             0 <= i < xs.len() as int
    ensures shrink_seq_nat(xs)[i].len() < xs.len()
{
    assert(shrink_seq_nat(xs)[i] == xs.remove(i));
    assert(xs.remove(i).len() == xs.len() - 1);
}

} // verus!