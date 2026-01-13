use vstd::prelude::*;

verus! {

pub open spec fn zip_with_m_option<A, B, C>(
    xs: Seq<A>,
    ys: Seq<B>,
    f: spec_fn(A, B) -> Option<C>
) -> Option<Seq<C>>
    decreases xs.len()
{
    if xs.len() == 0 || ys.len() == 0 {
        Option::Some(Seq::empty())
    } else {
        match (f(xs[0], ys[0]), zip_with_m_option(xs.skip(1), ys.skip(1), f)) {
            (Option::Some(c), Option::Some(cs)) => Option::Some(seq![c].add(cs)),
            _ => Option::None,
        }
    }
}

} // verus!