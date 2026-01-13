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


pub proof fn traverse_all_some<A, B>(xs: Seq<A>, f: spec_fn(A) -> Option<B>)
    requires forall|i: int| 0 <= i < xs.len() as int ==> f(xs[i]).is_some()
    ensures traverse_seq_option(xs, f).is_some()
    decreases xs.len()
{
    if xs.len() == 0 {
        // Base case
    } else {
        assert(f(xs[0]).is_some());
        traverse_all_some(xs.skip(1), f);
    }
}

} // verus!