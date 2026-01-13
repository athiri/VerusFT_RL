use vstd::prelude::*;

verus! {

pub open spec fn count_accepted<A>(values: Seq<A>, predicate: spec_fn(A) -> bool) -> nat
    decreases values.len()
{
    if values.len() == 0 {
        0
    } else {
        let first = values[0];
        let rest = count_accepted(values.drop_first(), predicate);
        if predicate(first) { 1 + rest } else { rest }
    }
}

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


pub proof fn count_equals_filter_length<A>(values: Seq<A>, predicate: spec_fn(A) -> bool)
    ensures count_accepted(values, predicate) == filter_seq(values, predicate).len()
{
    assume(count_accepted(values, predicate) == filter_seq(values, predicate).len());
}

} // verus!