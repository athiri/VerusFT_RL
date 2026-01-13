use vstd::prelude::*;

verus! {

pub open spec fn zip_with_m_option<A, B, C>(xs: Seq<A>, ys: Seq<B>, f: spec_fn(A, B) -> C) -> Option<Seq<C>>
    decreases xs.len()
{
    if xs.len() != ys.len() {
        None
    } else if xs.len() == 0 {
        Some(Seq::empty())
    } else {
        match zip_with_m_option(xs.drop_first(), ys.drop_first(), f) {
            None => None,
            Some(rest) => Some(seq![f(xs.first(), ys.first())].add(rest))
        }
    }
}

pub proof fn zip_with_m_option_len<A, B, C>(xs: Seq<A>, ys: Seq<B>, f: spec_fn(A, B) -> C, result: Seq<C>)
    requires zip_with_m_option(xs, ys, f) == Option::Some(result)
    ensures result.len() == xs.len()
    decreases xs.len()
{
    if xs.len() > 0 {
        match zip_with_m_option(xs.drop_first(), ys.drop_first(), f) {
            Some(rest) => {
                zip_with_m_option_len(xs.drop_first(), ys.drop_first(), f, rest);
            }
            None => {}
        }
    }
}

} // verus!
