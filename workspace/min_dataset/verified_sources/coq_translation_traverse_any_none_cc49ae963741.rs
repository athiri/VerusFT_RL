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


pub proof fn traverse_any_none<A, B>(xs: Seq<A>, f: spec_fn(A) -> Option<B>, k: int)
    requires 0 <= k < xs.len() as int,
             f(xs[k]) == Option::<B>::None
    ensures traverse_seq_option(xs, f) == Option::<Seq<B>>::None
    decreases xs.len()
{
    if xs.len() == 0 {
        // Vacuously true
    } else if k == 0 {
        assert(f(xs[0]) == Option::<B>::None);
    } else {
        traverse_any_none(xs.skip(1), f, k - 1);
    }
}

} // verus!