use vstd::prelude::*;

verus! {

pub open spec fn shrink_list_to_tails<A>(s: Seq<A>) -> Seq<Seq<A>>
    decreases s.len()
{
    if s.len() == 0 {
        seq![]
    } else {
        seq![s.drop_first()] + shrink_list_to_tails(s.drop_first())
    }
}

} // verus!