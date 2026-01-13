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


pub proof fn shrink_seq_singleton(n: nat)
    ensures shrink_seq_nat(seq![n]).len() == 1,
            shrink_seq_nat(seq![n])[0] =~= Seq::empty()
{
    let xs = seq![n];
    assert(xs.len() == 1);
    assert(shrink_seq_nat(xs) =~= seq![Seq::<nat>::empty()]);
}

} // verus!