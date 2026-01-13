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


pub proof fn shrink_seq_removes_elements(xs: Seq<nat>, i: int)
    requires xs.len() > 1,
             0 <= i < xs.len() as int
    ensures shrink_seq_nat(xs)[i].len() == xs.len() - 1
{
    let removes = Seq::new(xs.len(), |j: int| xs.remove(j));
    assert(shrink_seq_nat(xs) =~= removes);
    assert(removes[i] == xs.remove(i));
    assert(xs.remove(i).len() == xs.len() - 1);
}

} // verus!