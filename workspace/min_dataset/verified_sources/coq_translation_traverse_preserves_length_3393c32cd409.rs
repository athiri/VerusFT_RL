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


pub proof fn traverse_preserves_length<A, B>(xs: Seq<A>, f: spec_fn(A) -> Option<B>, result: Seq<B>)
    requires traverse_seq_option(xs, f) == Option::Some(result)
    ensures result.len() == xs.len()
    decreases xs.len()
{
    if xs.len() == 0 {
        assert(result =~= Seq::empty());
    } else {
        match (f(xs[0]), traverse_seq_option(xs.skip(1), f)) {
            (Option::Some(b), Option::Some(bs)) => {
                traverse_preserves_length(xs.skip(1), f, bs);
                assert(result =~= seq![b].add(bs));
                assert(result.len() == 1 + bs.len());
            }
            _ => {}  // Cannot happen given precondition
        }
    }
}

} // verus!