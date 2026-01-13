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

} // verus!