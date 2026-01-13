use vstd::prelude::*;

verus! {

pub open spec fn traverse_seq_option<A, B>(
    xs: Seq<A>,
    f: spec_fn(A) -> Option<B>
) -> Option<Seq<B>>
    decreases xs.len()
{
    if xs.len() == 0 {
        Option::Some(Seq::empty())
    } else {
        match (f(xs[0]), traverse_seq_option(xs.skip(1), f)) {
            (Option::Some(b), Option::Some(bs)) => Option::Some(seq![b].add(bs)),
            _ => Option::None,
        }
    }
}

} // verus!