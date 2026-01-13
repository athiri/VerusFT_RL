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


pub proof fn sequence_option_any_none(xs: Seq<Option<nat>>, k: int)
    requires 0 <= k < xs.len() as int,
             xs[k] == Option::<nat>::None
    ensures sequence_option(xs) == Option::<Seq<nat>>::None
    decreases xs.len()
{
    if xs.len() == 0 {
        // Vacuously true
    } else if k == 0 {
        assert(xs[0] == Option::<nat>::None);
    } else {
        // None is in the tail
        sequence_option_any_none(xs.skip(1), k - 1);
    }
}

} // verus!