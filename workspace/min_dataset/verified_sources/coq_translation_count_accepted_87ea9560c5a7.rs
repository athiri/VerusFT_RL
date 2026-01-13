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

} // verus!