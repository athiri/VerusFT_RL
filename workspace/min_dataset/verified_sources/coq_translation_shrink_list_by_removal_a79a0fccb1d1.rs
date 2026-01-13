use vstd::prelude::*;

verus! {

pub open spec fn shrink_list_by_removal<A>(s: Seq<A>) -> Seq<Seq<A>>
    decreases s.len()
{
    if s.len() == 0 {
        seq![]
    } else {
        seq![s.drop_first()] + seq![s.drop_last()]
    }
}

} // verus!