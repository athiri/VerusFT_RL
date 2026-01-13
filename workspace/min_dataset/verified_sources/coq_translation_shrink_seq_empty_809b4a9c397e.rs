use vstd::prelude::*;

verus! {

pub open spec fn shrink_seq_nat(xs: Seq<nat>) -> Seq<Seq<nat>>
    decreases xs.len()
{
    if xs.len() == 0 {
        Seq::empty()
    } else if xs.len() == 1 {
        // Single element: try empty sequence
        seq![Seq::empty()]
    } else {
        // Multiple elements: try removing each element
        let removes = Seq::new(xs.len(), |i: int| xs.remove(i));
        removes
    }
}


pub proof fn shrink_seq_empty()
    ensures shrink_seq_nat(Seq::empty()).len() == 0
{
    assert(shrink_seq_nat(Seq::empty()) =~= Seq::empty());
}

} // verus!