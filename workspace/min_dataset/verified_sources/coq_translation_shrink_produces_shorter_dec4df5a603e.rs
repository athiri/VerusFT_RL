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


pub proof fn shrink_produces_shorter<A>(s: Seq<A>)
    requires s.len() > 0
    ensures forall|i: int| 0 <= i < shrink_list_by_removal(s).len() ==>
        shrink_list_by_removal(s)[i].len() < s.len()
{
}

} // verus!