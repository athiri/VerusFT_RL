use vstd::prelude::*;

verus! {

pub open spec fn sequence_option<A>(xs: Seq<Option<A>>) -> Option<Seq<A>>
    decreases xs.len()
{
    if xs.len() == 0 {
        Option::Some(Seq::empty())
    } else {
        match (xs[0], sequence_option(xs.skip(1))) {
            (Option::Some(a), Option::Some(rest)) => Option::Some(seq![a].add(rest)),
            _ => Option::None,
        }
    }
}


pub proof fn sequence_option_all_some(xs: Seq<Option<nat>>)
    requires forall|i: int| 0 <= i < xs.len() as int ==> xs[i].is_some()
    ensures sequence_option(xs).is_some()
    decreases xs.len()
{
    if xs.len() == 0 {
        assert(sequence_option(xs) == Option::Some(Seq::<nat>::empty()));
    } else {
        assert(xs[0].is_some());
        sequence_option_all_some(xs.skip(1));
        assert(sequence_option(xs.skip(1)).is_some());
    }
}

} // verus!