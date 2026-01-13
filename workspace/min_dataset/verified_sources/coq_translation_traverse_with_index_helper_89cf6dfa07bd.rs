use vstd::prelude::*;

verus! {

pub open spec fn traverse_with_index_helper<A, B>(
    xs: Seq<A>,
    f: spec_fn(nat, A) -> Option<B>,
    start_idx: nat
) -> Option<Seq<B>>
    decreases xs.len()
{
    if xs.len() == 0 {
        Option::Some(Seq::empty())
    } else {
        match (f(start_idx, xs[0]), traverse_with_index_helper(xs.skip(1), f, start_idx + 1)) {
            (Option::Some(b), Option::Some(bs)) => Option::Some(seq![b].add(bs)),
            _ => Option::None,
        }
    }
}

} // verus!