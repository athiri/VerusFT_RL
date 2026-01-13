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

pub open spec fn sequence_seq_option<A>(xs: Seq<Option<A>>) -> Option<Seq<A>>
    decreases xs.len()
{
    if xs.len() == 0 {
        Option::Some(Seq::empty())
    } else {
        match (xs[0], sequence_seq_option(xs.skip(1))) {
            (Option::Some(a), Option::Some(rest)) => Option::Some(seq![a].add(rest)),
            _ => Option::None,
        }
    }
}


pub proof fn sequence_is_traverse_id<A>(xs: Seq<Option<A>>)
    ensures sequence_seq_option(xs) == traverse_seq_option(xs, |o: Option<A>| o)
    decreases xs.len()
{
    if xs.len() == 0 {
        assert(sequence_seq_option(xs) == Option::Some(Seq::<A>::empty()));
        assert(traverse_seq_option(xs, |o: Option<A>| o) == Option::Some(Seq::<A>::empty()));
    } else {
        sequence_is_traverse_id(xs.skip(1));
    }
}

} // verus!