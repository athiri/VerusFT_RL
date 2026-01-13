use vstd::prelude::*;

verus! {

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


pub open spec fn sequence_nested<A>(xss: Seq<Seq<Option<A>>>) -> Option<Seq<Seq<A>>>
    decreases xss.len()
{
    if xss.len() == 0 {
        Option::Some(Seq::empty())
    } else {
        match (sequence_seq_option(xss[0]), sequence_nested(xss.skip(1))) {
            (Option::Some(xs), Option::Some(rest)) => Option::Some(seq![xs].add(rest)),
            _ => Option::None,
        }
    }
}

} // verus!