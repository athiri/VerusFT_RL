use vstd::prelude::*;

verus! {

pub open spec fn filter_m_option<A>(xs: Seq<A>, p: spec_fn(A) -> Option<bool>) -> Option<Seq<A>>
    decreases xs.len()
{
    if xs.len() == 0 {
        Option::Some(Seq::empty())
    } else {
        match (p(xs[0]), filter_m_option(xs.skip(1), p)) {
            (Option::Some(keep), Option::Some(rest)) => {
                if keep {
                    Option::Some(seq![xs[0]].add(rest))
                } else {
                    Option::Some(rest)
                }
            }
            _ => Option::None,
        }
    }
}


pub proof fn filter_m_all_some<A>(xs: Seq<A>, p: spec_fn(A) -> Option<bool>)
    requires forall|i: int| 0 <= i < xs.len() as int ==> p(xs[i]).is_some()
    ensures filter_m_option(xs, p).is_some()
    decreases xs.len()
{
    if xs.len() == 0 {
        // Base case
    } else {
        filter_m_all_some(xs.skip(1), p);
    }
}

} // verus!