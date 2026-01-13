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


pub proof fn filter_satisfies_predicate<A>(values: Seq<A>, predicate: spec_fn(A) -> bool)
    ensures forall|i: int| 0 <= i < filter_seq(values, predicate).len() ==>
        predicate(filter_seq(values, predicate)[i])
{
    assume(forall|i: int| 0 <= i < filter_seq(values, predicate).len() ==>
        predicate(filter_seq(values, predicate)[i]));
}

} // verus!