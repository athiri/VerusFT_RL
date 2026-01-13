use vstd::prelude::*;

verus! {

pub open spec fn filter_seq<A>(values: Seq<A>, predicate: spec_fn(A) -> bool) -> Seq<A>
    decreases values.len()
{
    if values.len() == 0 {
        seq![]
    } else {
        let first = values[0];
        let rest = filter_seq(values.drop_first(), predicate);
        if predicate(first) {
            seq![first] + rest
        } else {
            rest
        }
    }
}


pub proof fn filter_length_bounded<A>(values: Seq<A>, predicate: spec_fn(A) -> bool)
    ensures filter_seq(values, predicate).len() <= values.len()
    decreases values.len()
{
    if values.len() > 0 {
        filter_length_bounded(values.drop_first(), predicate);
    }
}

} // verus!